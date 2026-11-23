use crate::state::LauncherInfo;
use dv_launcher::branding;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncWriteExt;
use tracing::info;

pub const GITHUB_RELEASES_API: &str =
    "https://api.github.com/repos/t041n/DV-Client/releases/latest";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_name: String,
    pub release_notes: String,
    pub published_at: Option<String>,
    pub html_url: String,
    pub download_url: Option<String>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    published_at: Option<String>,
    #[serde(default)]
    assets: Vec<GitHubAsset>,
}

fn parse_version_tuple(v: &str) -> (u64, u64, u64) {
    let clean = v.trim().trim_start_matches('v').trim_start_matches('V');
    let mut parts = clean.split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts
        .next()
        .and_then(|s| s.split('-').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (major, minor, patch)
}

fn is_version_newer(latest: &str, current: &str) -> bool {
    let (l_maj, l_min, l_pat) = parse_version_tuple(latest);
    let (c_maj, c_min, c_pat) = parse_version_tuple(current);
    (l_maj, l_min, l_pat) > (c_maj, c_min, c_pat)
}

#[tauri::command]
pub fn get_launcher_info() -> LauncherInfo {
    LauncherInfo {
        name: branding::APP_NAME.to_string(),
        version: branding::APP_VERSION.to_string(),
        author: "Digital Vigital".to_string(),
        repo_url: "https://github.com/t041n/DV-Client".to_string(),
    }
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn open_url(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateCheckResult, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let resp = client
        .get(GITHUB_RELEASES_API)
        .header(
            "User-Agent",
            format!("DigitalVigital-Launcher/{}", branding::APP_VERSION),
        )
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?;

    if !resp.status().is_success() {
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(UpdateCheckResult {
                has_update: false,
                current_version: branding::APP_VERSION.to_string(),
                latest_version: branding::APP_VERSION.to_string(),
                release_name: format!("Digital Vigital v{}", branding::APP_VERSION),
                release_notes: String::new(),
                published_at: None,
                html_url: "https://github.com/t041n/DV-Client/releases".to_string(),
                download_url: None,
            });
        }
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("GitHub API returned HTTP {}: {}", status, body));
    }

    let release: GitHubRelease = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub release payload: {}", e))?;

    let current_ver = branding::APP_VERSION;
    let latest_tag = release.tag_name.clone();
    let has_update = is_version_newer(&latest_tag, current_ver);

    // Look for platform-specific asset: .apk on Android, .exe/.msi/.zip on Windows, .AppImage/.deb/.tar.gz on Linux
    let download_url = {
        #[cfg(target_os = "android")]
        let asset = release.assets.iter().find(|a| a.name.ends_with(".apk"));

        #[cfg(target_os = "windows")]
        let asset = release.assets.iter().find(|a| a.name.ends_with(".exe") || a.name.ends_with(".msi") || a.name.ends_with(".zip"));

        #[cfg(target_os = "linux")]
        let asset = release.assets.iter().find(|a| a.name.ends_with(".AppImage") || a.name.ends_with(".deb") || a.name.ends_with(".tar.gz"));

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
        let asset = release.assets.iter().find(|a| a.name.ends_with(".exe") || a.name.ends_with(".zip") || a.name.ends_with(".tar.gz"));

        asset
            .map(|a| a.browser_download_url.clone())
            .or_else(|| Some(release.html_url.clone()))
    };

    let release_name = release
        .name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| format!("Digital Vigital {}", latest_tag));

    let release_notes = release.body.unwrap_or_default();

    info!(
        "Update check complete: current=v{}, latest={}, has_update={}",
        current_ver, latest_tag, has_update
    );

    Ok(UpdateCheckResult {
        has_update,
        current_version: current_ver.to_string(),
        latest_version: latest_tag,
        release_name,
        release_notes,
        published_at: release.published_at,
        html_url: release.html_url,
        download_url,
    })
}

#[derive(Serialize, Clone, Debug)]
pub struct UpdateProgressPayload {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f32,
    pub status: String,
}

#[tauri::command]
pub async fn download_and_install_update(
    download_url: String,
    app_handle: AppHandle,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("dv-launcher-updater")
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to initiate download: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Server returned HTTP {}", res.status()));
    }

    let total_bytes = res.content_length().unwrap_or(0);
    let temp_dir = std::env::temp_dir();

    let fname = download_url
        .split('/')
        .last()
        .and_then(|f| f.split('?').next())
        .unwrap_or("dv-launcher-update.exe");

    let dest_path = temp_dir.join(fname);
    let mut file = tokio::fs::File::create(&dest_path)
        .await
        .map_err(|e| format!("Failed to create destination file: {}", e))?;

    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;

    let _ = app_handle.emit(
        "update-download-progress",
        UpdateProgressPayload {
            downloaded_bytes: 0,
            total_bytes,
            percent: 0.0,
            status: "Starting download...".to_string(),
        },
    );

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| format!("Error downloading chunk: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Error writing chunk: {}", e))?;
        downloaded += chunk.len() as u64;

        let pct = if total_bytes > 0 {
            (downloaded as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        let _ = app_handle.emit(
            "update-download-progress",
            UpdateProgressPayload {
                downloaded_bytes: downloaded,
                total_bytes,
                percent: pct,
                status: format!("Downloading update... {:.1}%", pct),
            },
        );
    }

    file.flush().await.map_err(|e| e.to_string())?;

    let _ = app_handle.emit(
        "update-download-progress",
        UpdateProgressPayload {
            downloaded_bytes: downloaded,
            total_bytes,
            percent: 100.0,
            status: "Download complete! Opening installer...".to_string(),
        },
    );

    #[cfg(target_os = "windows")]
    {
        if fname.ends_with(".exe") || fname.ends_with(".msi") {
            let _ = std::process::Command::new(&dest_path).spawn();
        } else {
            let _ = dv_launcher::utils::open_path_in_explorer(&dest_path);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = dv_launcher::utils::open_path_in_explorer(&dest_path);
    }

    Ok(dest_path.to_string_lossy().to_string())
}
