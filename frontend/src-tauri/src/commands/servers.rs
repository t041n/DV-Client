use crate::state::AppState;
use dv_launcher::minecraft::server_ping::{ping_server as run_ping_server, ServerPingResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FavoriteServer {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub is_default: bool,
}

fn get_servers_file_path(data_dir: &std::path::Path) -> PathBuf {
    data_dir.join("servers.json")
}

fn default_favorite_servers() -> Vec<FavoriteServer> {
    vec![
        FavoriteServer {
            id: "hypixel".to_string(),
            name: "Hypixel Network".to_string(),
            host: "mc.hypixel.net".to_string(),
            port: 25565,
            is_default: true,
        },
        FavoriteServer {
            id: "minemen".to_string(),
            name: "Minemen Club".to_string(),
            host: "play.minemen.club".to_string(),
            port: 25565,
            is_default: true,
        },
        FavoriteServer {
            id: "cubecraft".to_string(),
            name: "CubeCraft Games".to_string(),
            host: "play.cubecraft.net".to_string(),
            port: 25565,
            is_default: true,
        },
    ]
}

#[tauri::command]
pub async fn ping_minecraft_server(
    host: String,
    port: Option<u16>,
    timeout_ms: Option<u64>,
) -> Result<ServerPingResult, String> {
    let p = port.unwrap_or(25565);
    let to = timeout_ms.unwrap_or(3500);
    run_ping_server(&host, p, to).await
}

#[tauri::command]
pub async fn get_favorite_servers(
    state: State<'_, AppState>,
) -> Result<Vec<FavoriteServer>, String> {
    let cm = state.config_mgr.lock().await;
    let path = get_servers_file_path(&cm.config.data_dir);

    if !path.exists() {
        let defaults = default_favorite_servers();
        let _ = fs::write(&path, serde_json::to_string_pretty(&defaults).unwrap_or_default());
        return Ok(defaults);
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            let mut list: Vec<FavoriteServer> = serde_json::from_str(&content).unwrap_or_else(|_| default_favorite_servers());
            if list.is_empty() {
                list = default_favorite_servers();
            }
            Ok(list)
        }
        Err(_) => Ok(default_favorite_servers()),
    }
}

#[tauri::command]
pub async fn save_favorite_server(
    id: Option<String>,
    name: String,
    host: String,
    port: Option<u16>,
    state: State<'_, AppState>,
) -> Result<FavoriteServer, String> {
    let cm = state.config_mgr.lock().await;
    let path = get_servers_file_path(&cm.config.data_dir);
    let mut list = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<Vec<FavoriteServer>>(&content).unwrap_or_else(|_| default_favorite_servers()),
        Err(_) => default_favorite_servers(),
    };

    let server_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let server_port = port.unwrap_or(25565);

    if let Some(existing) = list.iter_mut().find(|s| s.id == server_id) {
        existing.name = name.clone();
        existing.host = host.clone();
        existing.port = server_port;
        let result = existing.clone();
        let _ = fs::write(&path, serde_json::to_string_pretty(&list).unwrap_or_default());
        return Ok(result);
    }

    let new_server = FavoriteServer {
        id: server_id,
        name,
        host,
        port: server_port,
        is_default: false,
    };

    list.push(new_server.clone());
    let _ = fs::write(&path, serde_json::to_string_pretty(&list).unwrap_or_default());
    Ok(new_server)
}

#[tauri::command]
pub async fn delete_favorite_server(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let cm = state.config_mgr.lock().await;
    let path = get_servers_file_path(&cm.config.data_dir);
    let mut list = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<Vec<FavoriteServer>>(&content).unwrap_or_else(|_| default_favorite_servers()),
        Err(_) => default_favorite_servers(),
    };

    list.retain(|s| s.id != id);
    let _ = fs::write(&path, serde_json::to_string_pretty(&list).unwrap_or_default());
    Ok(())
}
