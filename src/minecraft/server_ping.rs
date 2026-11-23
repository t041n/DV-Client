//! Minecraft Server List Ping (SLP) & Latency Measurement.
//!
//! Implements the standard Minecraft Protocol Server List Ping handshake
//! to fetch real-time MOTD, online/max players, version string, favicon, and latency.

use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerPingResult {
    pub host: String,
    pub port: u16,
    pub online: bool,
    pub latency_ms: u64,
    pub version_name: String,
    pub protocol_version: i32,
    pub players_online: u32,
    pub players_max: u32,
    pub motd: String,
    pub favicon: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SlpVersion {
    #[serde(default)]
    name: String,
    #[serde(default)]
    protocol: i32,
}

#[derive(Deserialize, Debug)]
struct SlpPlayers {
    #[serde(default)]
    max: u32,
    #[serde(default)]
    online: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum SlpDescription {
    Plain(String),
    Object(serde_json::Value),
}

#[derive(Deserialize, Debug)]
struct SlpResponse {
    version: Option<SlpVersion>,
    players: Option<SlpPlayers>,
    description: Option<SlpDescription>,
    favicon: Option<String>,
}

fn write_varint(buf: &mut Vec<u8>, mut val: i32) {
    loop {
        if (val & !0x7F) == 0 {
            buf.push(val as u8);
            return;
        }
        buf.push(((val & 0x7F) | 0x80) as u8);
        val = ((val as u32) >> 7) as i32;
    }
}

async fn read_varint<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<i32, std::io::Error> {
    let mut num_read = 0;
    let mut result = 0;
    loop {
        let byte = reader.read_u8().await?;
        let value = (byte & 0x7F) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok(result)
}

fn clean_minecraft_formatting(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '§' {
            let _ = chars.next(); // Skip formatting code
        } else {
            out.push(c);
        }
    }
    out
}

fn extract_motd_text(desc: Option<SlpDescription>) -> String {
    match desc {
        Some(SlpDescription::Plain(s)) => clean_minecraft_formatting(&s),
        Some(SlpDescription::Object(v)) => {
            if let Some(text) = v.get("text").and_then(|t| t.as_str()) {
                let mut full = text.to_string();
                if let Some(extra) = v.get("extra").and_then(|e| e.as_array()) {
                    for item in extra {
                        if let Some(t) = item.get("text").and_then(|t| t.as_str()) {
                            full.push_str(t);
                        } else if let Some(t) = item.as_str() {
                            full.push_str(t);
                        }
                    }
                }
                clean_minecraft_formatting(&full)
            } else {
                clean_minecraft_formatting(&v.to_string())
            }
        }
        None => "A Minecraft Server".to_string(),
    }
}

/// Ping a Minecraft Java Edition server to retrieve live status & latency.
pub async fn ping_server(host: &str, port: u16, timeout_millis: u64) -> Result<ServerPingResult, String> {
    let addr = format!("{}:{}", host, port);
    let start_instant = Instant::now();

    let mut stream = timeout(
        Duration::from_millis(timeout_millis),
        TcpStream::connect(&addr),
    )
    .await
    .map_err(|_| format!("Connection timed out to {}", addr))?
    .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

    // Packet 0x00: Handshake
    // Protocol version: -1 (or 767), Server address, Server port, Next state: 1 (status)
    let mut handshake_data = Vec::new();
    handshake_data.push(0x00); // Packet ID
    write_varint(&mut handshake_data, -1); // Protocol version (-1 accepts any)
    write_varint(&mut handshake_data, host.len() as i32);
    handshake_data.extend_from_slice(host.as_bytes());
    handshake_data.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut handshake_data, 1); // State 1: status

    let mut handshake_packet = Vec::new();
    write_varint(&mut handshake_packet, handshake_data.len() as i32);
    handshake_packet.extend(handshake_data);

    // Packet 0x00: Status Request (length 1, id 0x00)
    let status_request = [0x01, 0x00];

    stream
        .write_all(&handshake_packet)
        .await
        .map_err(|e| format!("Failed to send handshake: {}", e))?;
    stream
        .write_all(&status_request)
        .await
        .map_err(|e| format!("Failed to send status request: {}", e))?;

    // Read Response Packet
    let _packet_len = timeout(Duration::from_millis(timeout_millis), read_varint(&mut stream))
        .await
        .map_err(|_| "Timed out reading packet length".to_string())?
        .map_err(|e| format!("Failed reading packet length: {}", e))?;

    let packet_id = read_varint(&mut stream)
        .await
        .map_err(|e| format!("Failed reading packet ID: {}", e))?;

    if packet_id != 0x00 {
        return Err(format!("Unexpected response packet ID: {}", packet_id));
    }

    let json_len = read_varint(&mut stream)
        .await
        .map_err(|e| format!("Failed reading JSON string length: {}", e))?;

    if json_len <= 0 || json_len > 1_000_000 {
        return Err(format!("Invalid response JSON length: {}", json_len));
    }

    let mut json_bytes = vec![0u8; json_len as usize];
    stream
        .read_exact(&mut json_bytes)
        .await
        .map_err(|e| format!("Failed reading JSON payload: {}", e))?;

    let latency_ms = start_instant.elapsed().as_millis() as u64;

    let json_str = String::from_utf8_lossy(&json_bytes);
    let slp: SlpResponse = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed parsing server JSON: {} ({})", e, json_str))?;

    let motd = extract_motd_text(slp.description);
    let version_name = slp.version.as_ref().map(|v| v.name.clone()).unwrap_or_else(|| "Unknown".to_string());
    let protocol_version = slp.version.as_ref().map(|v| v.protocol).unwrap_or(0);
    let players_online = slp.players.as_ref().map(|p| p.online).unwrap_or(0);
    let players_max = slp.players.as_ref().map(|p| p.max).unwrap_or(0);

    Ok(ServerPingResult {
        host: host.to_string(),
        port,
        online: true,
        latency_ms,
        version_name,
        protocol_version,
        players_online,
        players_max,
        motd,
        favicon: slp.favicon,
    })
}
