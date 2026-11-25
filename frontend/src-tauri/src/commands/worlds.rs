use crate::AppState;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::State;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(serde::Serialize, Clone, Debug)]
pub struct WorldSaveDto {
    pub folder_name: String,
    pub name: String,
    pub last_played_timestamp: Option<u64>,
    pub size_bytes: u64,
    pub icon_base64: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct ScreenshotDto {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
    pub modified_timestamp: Option<u64>,
    pub thumbnail_base64: Option<String>,
}

fn fast_base64_encode(data: &[u8]) -> String {
    const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(B64[b0 >> 2] as char);
        out.push(B64[((b0 & 3) << 4) | (b1 >> 4)] as char);
        if chunk.len() > 1 {
            out.push(B64[((b1 & 15) << 2) | (b2 >> 6)] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64[b2 & 63] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn dir_size(path: &Path) -> u64 {
    use rayon::prelude::*;
    if let Ok(entries) = std::fs::read_dir(path) {
        let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.into_par_iter().map(|entry| {
            let p = entry.path();
            if p.is_dir() {
                dir_size(&p)
            } else if let Ok(meta) = entry.metadata() {
                meta.len()
            } else {
                0
            }
        }).sum()
    } else {
        0
    }
}

fn open_path_native(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("explorer.exe");
        cmd.arg(path);
        if cmd.spawn().is_ok() {
            return Ok(());
        }
    }
    open::that_detached(path)
        .or_else(|_| open::that(path))
        .map_err(|e| format!("Could not open folder: {}", e))
}

#[tauri::command]
pub async fn get_instance_worlds(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<WorldSaveDto>, String> {
    let (saves_dir, alt_saves_dir) = {
        let mgr = state.instance_mgr.lock().await;
        let inst = mgr
            .instances
            .get(&instance_id)
            .ok_or_else(|| "Instance not found".to_string())?;

        let saves = inst.game_dir(&mgr.instances_dir).join("saves");
        let alt = inst.instance_dir(&mgr.instances_dir).join("saves");
        (saves, alt)
    };

    let _ = std::fs::create_dir_all(&saves_dir);

    tokio::task::spawn_blocking(move || {
        use rayon::prelude::*;
        let mut seen_folders = std::collections::HashSet::new();
        let mut candidates = Vec::new();

        let scan_dirs = vec![saves_dir, alt_saves_dir];
        for dir in scan_dirs {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if !path.is_dir() {
                        continue;
                    }
                    let folder_name = entry.file_name().to_string_lossy().to_string();
                    if folder_name.starts_with('.') || seen_folders.contains(&folder_name) {
                        continue;
                    }
                    seen_folders.insert(folder_name.clone());
                    candidates.push((folder_name, path));
                }
            }
        }

        let mut worlds: Vec<WorldSaveDto> = candidates
            .into_par_iter()
            .map(|(folder_name, path)| {
                let meta = path.metadata().ok();
                let modified = meta.and_then(|m| m.modified().ok()).and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
                });

                let size = dir_size(&path);

                let icon_path = path.join("icon.png");
                let icon_base64 = if icon_path.exists() {
                    std::fs::read(&icon_path)
                        .ok()
                        .map(|b| format!("data:image/png;base64,{}", fast_base64_encode(&b)))
                } else {
                    None
                };

                let world_name = folder_name.replace('_', " ");

                WorldSaveDto {
                    folder_name,
                    name: world_name,
                    last_played_timestamp: modified,
                    size_bytes: size,
                    icon_base64,
                }
            })
            .collect();

        worlds.sort_by(|a, b| b.last_played_timestamp.cmp(&a.last_played_timestamp));
        Ok(worlds)
    })
    .await
    .map_err(|e| format!("Failed to scan worlds: {}", e))?
}

#[tauri::command]
pub async fn open_instance_saves(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let saves_dir = inst.game_dir(&mgr.instances_dir).join("saves");
    std::fs::create_dir_all(&saves_dir).map_err(|e| e.to_string())?;

    open_path_native(&saves_dir)
}

#[tauri::command]
pub async fn open_instance_screenshots(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let shots_dir = inst.game_dir(&mgr.instances_dir).join("screenshots");
    std::fs::create_dir_all(&shots_dir).map_err(|e| e.to_string())?;

    open_path_native(&shots_dir)
}

#[tauri::command]
pub async fn backup_world(
    instance_id: String,
    world_folder: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let game_dir = inst.game_dir(&mgr.instances_dir);
    let mut world_path = game_dir.join("saves").join(&world_folder);
    if !world_path.exists() {
        let alt = inst.instance_dir(&mgr.instances_dir).join("saves").join(&world_folder);
        if alt.exists() {
            world_path = alt;
        } else {
            return Err("World save folder does not exist".to_string());
        }
    }

    let backups_dir = game_dir.join("backups");
    std::fs::create_dir_all(&backups_dir).map_err(|e| e.to_string())?;

    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let zip_filename = format!("{}_{}.zip", world_folder, now_sec);
    let zip_dest = backups_dir.join(&zip_filename);

    let zip_file = File::create(&zip_dest).map_err(|e| e.to_string())?;
    let mut zip_writer = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    fn add_dir_to_zip<P: AsRef<Path>>(
        writer: &mut ZipWriter<File>,
        root: &Path,
        current: P,
        options: SimpleFileOptions,
    ) -> Result<(), String> {
        let cur = current.as_ref();
        let entries = std::fs::read_dir(cur).map_err(|e| e.to_string())?;
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            let relative = p
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .replace('\\', "/");

            if p.is_dir() {
                writer
                    .add_directory(&format!("{}/", relative), options)
                    .map_err(|e| e.to_string())?;
                add_dir_to_zip(writer, root, &p, options)?;
            } else {
                writer
                    .start_file(&relative, options)
                    .map_err(|e| e.to_string())?;
                let mut f = File::open(&p).map_err(|e| e.to_string())?;
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).map_err(|e| e.to_string())?;
                use std::io::Write;
                writer.write_all(&buf).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    add_dir_to_zip(&mut zip_writer, &world_path, &world_path, options)?;
    zip_writer.finish().map_err(|e| e.to_string())?;

    Ok(zip_dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn delete_world(
    instance_id: String,
    world_folder: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let p1 = inst.game_dir(&mgr.instances_dir).join("saves").join(&world_folder);
    if p1.exists() {
        let _ = std::fs::remove_dir_all(&p1);
    }
    let p2 = inst.instance_dir(&mgr.instances_dir).join("saves").join(&world_folder);
    if p2.exists() {
        let _ = std::fs::remove_dir_all(&p2);
    }
    Ok(())
}

#[tauri::command]
pub async fn import_world_dialog(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let picked = tokio::task::spawn_blocking(|| {
        rfd::FileDialog::new()
            .set_title("Select World Backup / Save (.zip)")
            .add_filter("World Archive (*.zip)", &["zip"])
            .pick_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    let file_path = match picked {
        Some(p) => p,
        None => return Ok(None),
    };

    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let saves_dir = inst.game_dir(&mgr.instances_dir).join("saves");
    std::fs::create_dir_all(&saves_dir).map_err(|e| e.to_string())?;

    let file = File::open(&file_path).map_err(|e| format!("Cannot open zip file: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Invalid zip archive: {}", e))?;

    // Check if zip root directly has level.dat or is nested in a folder
    let mut root_has_level_dat = false;
    for i in 0..archive.len() {
        if let Ok(entry) = archive.by_index(i) {
            let name = entry.name().replace('\\', "/");
            if name == "level.dat" || name == "/level.dat" {
                root_has_level_dat = true;
                break;
            }
        }
    }

    let default_name = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Imported_World".to_string());

    let target_dir = if root_has_level_dat {
        saves_dir.join(&default_name)
    } else {
        saves_dir.clone()
    };
    std::fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok(Some(default_name))
}

#[tauri::command]
pub async fn import_vanilla_worlds(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let vanilla_saves: Option<PathBuf> = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .ok()
            .map(|a| Path::new(&a).join(".minecraft").join("saves"))
    } else if cfg!(target_os = "macos") {
        dirs::home_dir().map(|h| h.join("Library/Application Support/minecraft/saves"))
    } else {
        dirs::home_dir().map(|h| h.join(".minecraft/saves"))
    };

    let vanilla_dir = match vanilla_saves {
        Some(d) if d.exists() => d,
        _ => return Ok(0),
    };

    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let target_saves = inst.game_dir(&mgr.instances_dir).join("saves");
    std::fs::create_dir_all(&target_saves).map_err(|e| e.to_string())?;

    let mut imported = 0;
    if let Ok(entries) = std::fs::read_dir(&vanilla_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let src_path = entry.path();
            if src_path.is_dir() {
                let name = entry.file_name();
                let dst_path = target_saves.join(&name);
                if !dst_path.exists() {
                    fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
                        std::fs::create_dir_all(dst)?;
                        for entry in std::fs::read_dir(src)? {
                            let entry = entry?;
                            let ty = entry.file_type()?;
                            if ty.is_dir() {
                                copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                            } else {
                                std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
                            }
                        }
                        Ok(())
                    }
                    if copy_dir_all(&src_path, &dst_path).is_ok() {
                        imported += 1;
                    }
                }
            }
        }
    }
    Ok(imported)
}

#[tauri::command]
pub async fn get_instance_screenshots(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ScreenshotDto>, String> {
    let screenshots_dir = {
        let mgr = state.instance_mgr.lock().await;
        let inst = mgr
            .instances
            .get(&instance_id)
            .ok_or_else(|| "Instance not found".to_string())?;

        inst.game_dir(&mgr.instances_dir).join("screenshots")
    };

    let _ = std::fs::create_dir_all(&screenshots_dir);

    tokio::task::spawn_blocking(move || {
        use rayon::prelude::*;
        let entries = match std::fs::read_dir(&screenshots_dir) {
            Ok(e) => e,
            Err(_) => return Ok(Vec::new()),
        };

        let candidate_paths: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let p = entry.path();
                if p.is_file() {
                    let fname = entry.file_name().to_string_lossy().to_string();
                    if fname.to_lowercase().ends_with(".png") {
                        return Some((fname, p));
                    }
                }
                None
            })
            .collect();

        let mut screenshots: Vec<ScreenshotDto> = candidate_paths
            .into_par_iter()
            .map(|(fname, path)| {
                let meta = path.metadata().ok();
                let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = meta.and_then(|m| m.modified().ok()).and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
                });

                let thumbnail_base64 = if size < 4 * 1024 * 1024 {
                    std::fs::read(&path)
                        .ok()
                        .map(|b| format!("data:image/png;base64,{}", fast_base64_encode(&b)))
                } else {
                    None
                };

                ScreenshotDto {
                    filename: fname,
                    path: path.to_string_lossy().to_string(),
                    size_bytes: size,
                    modified_timestamp: modified,
                    thumbnail_base64,
                }
            })
            .collect();

        screenshots.sort_by(|a, b| b.modified_timestamp.cmp(&a.modified_timestamp));
        Ok(screenshots)
    })
    .await
    .map_err(|e| format!("Failed to read screenshots: {}", e))?
}

#[tauri::command]
pub async fn delete_screenshot(
    instance_id: String,
    filename: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let path = inst.game_dir(&mgr.instances_dir).join("screenshots").join(&filename);
    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }
    Ok(())
}
