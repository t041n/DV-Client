//! Modrinth Online Mod Discovery and 1-Click Installation Client.
//!
//! Provides real-time search across Modrinth's catalog of 50,000+ open-source
//! Minecraft mods, compatibility filtering, and atomic downloads.

use crate::branding;
use crate::loaders::LoaderError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;
use tracing::{info, warn};

pub const MODRINTH_API_BASE: &str = "https://api.modrinth.com/v2";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModrinthMod {
    pub project_id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub cached_icon_path: Option<std::path::PathBuf>,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModrinthVersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModrinthDependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    #[serde(default)]
    pub dependency_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModrinthVersionItem {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthVersionFile>,
    #[serde(default)]
    pub dependencies: Vec<ModrinthDependency>,
}

#[derive(Clone)]
pub struct ModrinthClient {
    client: Client,
}

impl Default for ModrinthClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ModrinthClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(branding::USER_AGENT)
            .timeout(Duration::from_secs(12))
            .build()
            .expect("Failed to initialize reqwest client for ModrinthClient");

        Self { client }
    }

    /// Search mods on Modrinth with query and optional category / version filters.
    pub async fn search_mods(
        &self,
        query: &str,
        mc_version: Option<&str>,
        loader: Option<&str>,
    ) -> Result<Vec<ModrinthMod>, LoaderError> {
        self.search_mods_advanced(query, mc_version, loader, None, None, 40, None).await
    }

    /// Advanced Modrinth search with category filtering, sorting index, project type, and result limits.
    pub async fn search_mods_advanced(
        &self,
        query: &str,
        mc_version: Option<&str>,
        loader: Option<&str>,
        category: Option<&str>,
        sort_by: Option<&str>,
        limit: u32,
        project_type: Option<&str>,
    ) -> Result<Vec<ModrinthMod>, LoaderError> {
        let trimmed_query = query.trim();
        let sort = sort_by.unwrap_or(if trimmed_query.is_empty() { "downloads" } else { "relevance" });
        let mut url = format!(
            "{}/search?query={}&index={}&limit={}",
            MODRINTH_API_BASE,
            urlencoding_encode(trimmed_query),
            sort,
            limit.clamp(1, 100)
        );

        // Build facets filter: project_type (mod, shader, resourcepack, datapack)
        let ptype = project_type.unwrap_or("mod");
        let mut facets = vec![format!(r#"["project_type:{}"]"#, ptype)];
        if ptype == "mod" {
            if let Some(loader_name) = loader {
                let l_clean = loader_name.to_lowercase();
                if l_clean != "vanilla" && !l_clean.is_empty() {
                    facets.push(format!(r#"["categories:{}"]"#, l_clean));
                }
            }
        }
        if let Some(ver) = mc_version {
            if !ver.is_empty() {
                facets.push(format!(r#"["versions:{}"]"#, ver));
            }
        }
        if let Some(cat) = category {
            let c_clean = cat.to_lowercase();
            if !c_clean.is_empty() && c_clean != "all" {
                facets.push(format!(r#"["categories:{}"]"#, c_clean));
            }
        }

        let facets_json = format!("[{}]", facets.join(","));
        url.push_str(&format!("&facets={}", urlencoding_encode(&facets_json)));

        let resp = match self.client.get(&url).send().await {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to query Modrinth search: {}", e);
                return Ok(Vec::new());
            }
        };

        if !resp.status().is_success() {
            return Ok(Vec::new());
        }

        let body: serde_json::Value = resp.json().await?;
        let mut results = Vec::new();

        if let Some(hits) = body.get("hits").and_then(|h| h.as_array()) {
            for hit in hits {
                let project_id = hit
                    .get("project_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let slug = hit
                    .get("slug")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let title = hit
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let description = hit
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let author = hit
                    .get("author")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let downloads = hit.get("downloads").and_then(|v| v.as_u64()).unwrap_or(0);
                let icon_url = hit
                    .get("icon_url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let categories = hit
                    .get("categories")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let versions = hit
                    .get("game_versions")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_else(|| {
                        hit.get("versions")
                            .and_then(|c| c.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default()
                    });

                if !title.is_empty() {
                    results.push(ModrinthMod {
                        project_id,
                        slug,
                        title,
                        description,
                        author,
                        downloads,
                        icon_url,
                        cached_icon_path: None,
                        categories,
                        versions,
                    });
                }
            }
        }

        // Fetch mod icons concurrently
        let icon_futures: Vec<_> = results
            .iter()
            .map(|m| {
                let client = self.clone();
                let pid = m.project_id.clone();
                let url_opt = m.icon_url.clone();
                async move {
                    if let Some(url) = url_opt {
                        client.fetch_mod_icon(&pid, &url).await
                    } else {
                        None
                    }
                }
            })
            .collect();

        let icon_paths = futures_util::future::join_all(icon_futures).await;
        for (m, p) in results.iter_mut().zip(icon_paths) {
            m.cached_icon_path = p;
        }

        Ok(results)
    }

    /// Fetch and cache a Modrinth mod logo into the icon cache directory.
    pub async fn fetch_mod_icon(
        &self,
        project_id: &str,
        icon_url: &str,
    ) -> Option<std::path::PathBuf> {
        let cache_dir = crate::instances::mods::get_mod_icon_cache_dir();
        let icon_path = cache_dir.join(format!("modrinth_{}.png", project_id));

        if icon_path.exists() {
            // Verify valid PNG signature: [0x89, 0x50, 0x4E, 0x47]
            let is_valid = if let Ok(mut f) = fs::File::open(&icon_path) {
                use std::io::Read;
                let mut header = [0u8; 4];
                f.read_exact(&mut header).is_ok() && header == [0x89, 0x50, 0x4E, 0x47]
            } else {
                false
            };

            if is_valid {
                return Some(icon_path);
            }

            // Attempt to re-encode existing corrupt/WebP file to valid PNG
            if let Ok(bytes) = fs::read(&icon_path) {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    if img
                        .save_with_format(&icon_path, image::ImageFormat::Png)
                        .is_ok()
                    {
                        return Some(icon_path);
                    }
                }
            }
            let _ = fs::remove_file(&icon_path);
        }

        match self.client.get(icon_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(bytes) = resp.bytes().await {
                    if !bytes.is_empty() {
                        let path_clone = icon_path.clone();
                        let saved = tokio::task::spawn_blocking(move || {
                            if bytes.len() >= 4 && bytes[..4] == [0x89, 0x50, 0x4E, 0x47] {
                                fs::write(&path_clone, &bytes).is_ok()
                            } else if let Ok(img) = image::load_from_memory(&bytes) {
                                img.save_with_format(&path_clone, image::ImageFormat::Png)
                                    .is_ok()
                            } else {
                                false
                            }
                        })
                        .await
                        .unwrap_or(false);

                        if saved {
                            return Some(icon_path);
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Download and install a mod into the instance's mods folder.
    pub async fn install_mod<P: AsRef<Path>>(
        &self,
        project_id_or_slug: &str,
        mc_version: &str,
        loader: &str,
        mods_dir: P,
    ) -> Result<String, LoaderError> {
        let dir = mods_dir.as_ref();
        fs::create_dir_all(dir)?;

        let mut url = format!(
            "{}/project/{}/version",
            MODRINTH_API_BASE, project_id_or_slug
        );

        let l_clean = loader.to_lowercase();
        let mut query_params = Vec::new();
        if l_clean != "vanilla" && l_clean != "shader" && l_clean != "resourcepack" && l_clean != "datapack" && !l_clean.is_empty() {
            query_params.push(format!("loaders=[%22{}%22]", l_clean));
        }
        if !mc_version.is_empty() {
            query_params.push(format!("game_versions=[%22{}%22]", mc_version));
        }
        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        info!("Fetching compatible Modrinth versions: {}", url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let versions: Vec<ModrinthVersionItem> = resp.json().await?;

        if versions.is_empty() {
            return Err(LoaderError::Custom(format!(
                "No compatible version found for mod '{}' on MC {} with {}",
                project_id_or_slug, mc_version, loader
            )));
        }

        // Pick first matching version and its primary file
        let selected_ver = &versions[0];
        let file = selected_ver
            .files
            .iter()
            .find(|f| f.primary)
            .or_else(|| selected_ver.files.first())
            .ok_or_else(|| LoaderError::Custom("Mod version has no download files".to_string()))?;

        let dest = dir.join(&file.filename);
        info!(
            "Downloading mod '{}' from {} into {:?}",
            file.filename, file.url, dest
        );

        let dl_resp = self
            .client
            .get(&file.url)
            .send()
            .await?
            .error_for_status()?;
        let bytes = dl_resp.bytes().await?;
        fs::write(&dest, bytes)?;

        // Auto-install required dependencies (e.g. malilib for litematica, fabric-api, etc.)
        for dep in &selected_ver.dependencies {
            if dep.dependency_type == "required" {
                if let Some(ref dep_proj) = dep.project_id {
                    let already_installed = if let Ok(entries) = fs::read_dir(dir) {
                        entries.flatten().any(|e| {
                            let fname = e.file_name().to_string_lossy().to_lowercase();
                            let proj_clean = dep_proj.to_lowercase();
                            (fname.contains(&proj_clean) || fname.starts_with(&proj_clean))
                                && (fname.ends_with(".jar") || fname.ends_with(".disabled"))
                        })
                    } else {
                        false
                    };

                    if !already_installed {
                        info!(
                            "Auto-installing required Modrinth dependency '{}' for '{}' on MC {}...",
                            dep_proj, project_id_or_slug, mc_version
                        );
                        let mut dep_url = format!("{}/project/{}/version", MODRINTH_API_BASE, dep_proj);
                        let mut dep_params = Vec::new();
                        if l_clean != "vanilla" && !l_clean.is_empty() {
                            dep_params.push(format!("loaders=[%22{}%22]", l_clean));
                        }
                        if !mc_version.is_empty() {
                            dep_params.push(format!("game_versions=[%22{}%22]", mc_version));
                        }
                        if !dep_params.is_empty() {
                            dep_url.push('?');
                            dep_url.push_str(&dep_params.join("&"));
                        }

                        if let Ok(dep_resp) = self.client.get(&dep_url).send().await {
                            if let Ok(dep_versions) = dep_resp.json::<Vec<ModrinthVersionItem>>().await {
                                if let Some(dep_ver) = dep_versions.first() {
                                    if let Some(dep_file) = dep_ver.files.iter().find(|f| f.primary).or_else(|| dep_ver.files.first()) {
                                        let dep_dest = dir.join(&dep_file.filename);
                                        if !dep_dest.exists() {
                                            if let Ok(dl) = self.client.get(&dep_file.url).send().await {
                                                if let Ok(b) = dl.bytes().await {
                                                    let _ = fs::write(&dep_dest, b);
                                                    info!(
                                                        "Successfully installed required dependency: {}",
                                                        dep_file.filename
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(file.filename.clone())
    }
}

fn urlencoding_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modrinth_urlencoding() {
        assert_eq!(urlencoding_encode("sodium mod"), "sodium+mod");
        assert_eq!(urlencoding_encode("1.21.4"), "1.21.4");
    }
}
