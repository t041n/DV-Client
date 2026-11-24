use crate::AppState;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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
    let mut total = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() {
                total += dir_size(&p);
            } else if let Ok(meta) = entry.metadata() {
                total += meta.len();
            }
        }
    }
    total
}

#[tauri::command]
pub async fn get_instance_worlds(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<WorldSaveDto>, String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let saves_dir = inst.game_dir(&mgr.instances_dir).join("saves");
    if !saves_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = match std::fs::read_dir(&saves_dir) {
        Ok(e) => e,
        Err(_) => return Ok(Vec::new()),
    };

    let mut worlds = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let folder_name = entry.file_name().to_string_lossy().to_string();
        if folder_name.starts_with('.') {
            continue;
        }

        let meta = entry.metadata().ok();
        let modified = meta.and_then(|m| m.modified().ok()).and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
        });

        let size = dir_size(&path);

        // Check for icon.png in save folder
        let icon_path = path.join("icon.png");
        let icon_base64 = if icon_path.exists() {
            std::fs::read(&icon_path)
                .ok()
                .map(|b| format!("data:image/png;base64,{}", fast_base64_encode(&b)))
        } else {
            None
        };

        let world_name = folder_name.replace('_', " ");

        worlds.push(WorldSaveDto {
            folder_name,
            name: world_name,
            last_played_timestamp: modified,
            size_bytes: size,
            icon_base64,
        });
    }

    worlds.sort_by(|a, b| b.last_played_timestamp.cmp(&a.last_played_timestamp));
    Ok(worlds)
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
    let world_path = game_dir.join("saves").join(&world_folder);
    if !world_path.exists() {
        return Err("World save folder does not exist".to_string());
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

    let world_path = inst.game_dir(&mgr.instances_dir).join("saves").join(&world_folder);
    if world_path.exists() {
        std::fs::remove_dir_all(&world_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_instance_screenshots(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ScreenshotDto>, String> {
    let mgr = state.instance_mgr.lock().await;
    let inst = mgr
        .instances
        .get(&instance_id)
        .ok_or_else(|| "Instance not found".to_string())?;

    let screenshots_dir = inst.game_dir(&mgr.instances_dir).join("screenshots");
    if !screenshots_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = match std::fs::read_dir(&screenshots_dir) {
        Ok(e) => e,
        Err(_) => return Ok(Vec::new()),
    };

    let mut screenshots = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let fname = entry.file_name().to_string_lossy().to_string();
            if fname.to_lowercase().ends_with(".png") {
                let meta = entry.metadata().ok();
                let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = meta.and_then(|m| m.modified().ok()).and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
                });

                // Thumbnail: read up to 4MB base64 for in-app preview
                let thumbnail_base64 = if size < 4 * 1024 * 1024 {
                    std::fs::read(&path)
                        .ok()
                        .map(|b| format!("data:image/png;base64,{}", fast_base64_encode(&b)))
                } else {
                    None
                };

                screenshots.push(ScreenshotDto {
                    filename: fname,
                    path: path.to_string_lossy().to_string(),
                    size_bytes: size,
                    modified_timestamp: modified,
                    thumbnail_base64,
                });
            }
        }
    }

    screenshots.sort_by(|a, b| b.modified_timestamp.cmp(&a.modified_timestamp));
    Ok(screenshots)
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
