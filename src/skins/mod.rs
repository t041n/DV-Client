//! Minecraft Player Skin and Cape Management System.
//!
//! Provides:
//! - 64x64 and 64x32 skin rendering and front model compositing with overlay layers
//! - Cape rendering and customizable cape presets (DV Crimson, Founder, Migrator, Custom)
//! - Online skin fetching via Mojang / MC-Heads session infrastructure
//! - Local skin & cape caching and upload support for Offline & Microsoft profiles

use image::{ImageBuffer, Rgba, RgbaImage};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum SkinError {
    #[error("I/O error handling skin or cape file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image decoding error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Invalid skin dimensions: expected 64x64 or 64x32, got {0}x{1}")]
    InvalidDimensions(u32, u32),
}

/// Skin model type (Classic 4px arm vs Slim 3px arm).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkinModel {
    Classic,
    Slim,
}

/// Directory paths for skins and capes.
pub fn get_skins_dir() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("digitalvigital");
    let dir = base.join("skins");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn get_capes_dir() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("digitalvigital");
    let dir = base.join("capes");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Universal skin path shared across all player profiles.
pub fn get_universal_skin_path() -> PathBuf {
    get_skins_dir().join("universal_skin_raw.png")
}

pub fn get_universal_skin_preview_path() -> PathBuf {
    get_skins_dir().join("universal_skin_preview.png")
}

/// Universal cape path shared across all player profiles.
pub fn get_universal_cape_path() -> PathBuf {
    get_capes_dir().join("universal_cape.png")
}

pub fn get_universal_cape_preview_path() -> PathBuf {
    get_capes_dir().join("universal_cape_preview.png")
}

/// Composites a crisp 160x320 front-view player model preview from a 64x64 or 64x32 skin PNG.
/// Includes base body parts and second-layer overlays (hat, jacket, sleeves, pants).
pub fn render_skin_preview<P: AsRef<Path>>(skin_path: P) -> Result<RgbaImage, SkinError> {
    let skin_img = image::open(skin_path.as_ref())?.to_rgba8();
    let (w, h) = skin_img.dimensions();

    if w != 64 || (h != 64 && h != 32) {
        return Err(SkinError::InvalidDimensions(w, h));
    }

    // 16x32 canvas representing the front Minecraft character model
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(16, 32, Rgba([0, 0, 0, 0]));

    let blit_part = |dest: &mut RgbaImage,
                     src: &RgbaImage,
                     src_x: u32,
                     src_y: u32,
                     w: u32,
                     h: u32,
                     dest_x: u32,
                     dest_y: u32| {
        for dy in 0..h {
            for dx in 0..w {
                let px = src.get_pixel(src_x + dx, src_y + dy);
                if px[3] > 10 {
                    let d = dest.get_pixel_mut(dest_x + dx, dest_y + dy);
                    if px[3] == 255 || d[3] == 0 {
                        *d = *px;
                    } else {
                        // Alpha blend
                        let a = px[3] as f32 / 255.0;
                        let inv_a = 1.0 - a;
                        d[0] = (px[0] as f32 * a + d[0] as f32 * inv_a) as u8;
                        d[1] = (px[1] as f32 * a + d[1] as f32 * inv_a) as u8;
                        d[2] = (px[2] as f32 * a + d[2] as f32 * inv_a) as u8;
                        d[3] = 255;
                    }
                }
            }
        }
    };

    // 1. Head (8x8) at (4, 0)
    blit_part(&mut canvas, &skin_img, 8, 8, 8, 8, 4, 0);
    // Head Overlay (Hat) at (4, 0)
    blit_part(&mut canvas, &skin_img, 40, 8, 8, 8, 4, 0);

    // 2. Torso (8x12) at (4, 8)
    blit_part(&mut canvas, &skin_img, 20, 20, 8, 12, 4, 8);
    if h == 64 {
        // Torso Overlay (Jacket) at (4, 8)
        blit_part(&mut canvas, &skin_img, 20, 36, 8, 12, 4, 8);
    }

    // 3. Right Arm (4x12) at (0, 8)
    blit_part(&mut canvas, &skin_img, 44, 20, 4, 12, 0, 8);
    if h == 64 {
        // Right Arm Overlay
        blit_part(&mut canvas, &skin_img, 44, 36, 4, 12, 0, 8);
    }

    // 4. Left Arm (4x12) at (12, 8)
    if h == 64 {
        blit_part(&mut canvas, &skin_img, 36, 52, 4, 12, 12, 8);
        blit_part(&mut canvas, &skin_img, 52, 52, 4, 12, 12, 8);
    } else {
        // Legacy: Mirrored right arm
        for dy in 0..12 {
            for dx in 0..4 {
                let px = skin_img.get_pixel(44 + (3 - dx), 20 + dy);
                if px[3] > 10 {
                    canvas.put_pixel(12 + dx, 8 + dy, *px);
                }
            }
        }
    }

    // 5. Right Leg (4x12) at (4, 20)
    blit_part(&mut canvas, &skin_img, 4, 20, 4, 12, 4, 20);
    if h == 64 {
        blit_part(&mut canvas, &skin_img, 4, 36, 4, 12, 4, 20);
    }

    // 6. Left Leg (4x12) at (8, 20)
    if h == 64 {
        blit_part(&mut canvas, &skin_img, 20, 52, 4, 12, 8, 20);
        blit_part(&mut canvas, &skin_img, 4, 52, 4, 12, 8, 20);
    } else {
        // Legacy: Mirrored right leg
        for dy in 0..12 {
            for dx in 0..4 {
                let px = skin_img.get_pixel(4 + (3 - dx), 20 + dy);
                if px[3] > 10 {
                    canvas.put_pixel(8 + dx, 20 + dy, *px);
                }
            }
        }
    }

    // Upscale canvas nearest-neighbor by 10x to 160x320 for ultra-crisp display in the UI
    let scale = 10;
    let mut high_res = ImageBuffer::from_pixel(16 * scale, 32 * scale, Rgba([0, 0, 0, 0]));
    for y in 0..32 {
        for x in 0..16 {
            let px = canvas.get_pixel(x, y);
            if px[3] > 0 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        high_res.put_pixel(x * scale + sx, y * scale + sy, *px);
                    }
                }
            }
        }
    }

    Ok(high_res)
}

/// Generates a preview image of a cape (100x160 px).
pub fn render_cape_preview<P: AsRef<Path>>(cape_path: P) -> Result<RgbaImage, SkinError> {
    let cape_img = image::open(cape_path.as_ref())?.to_rgba8();
    let (cw, ch) = cape_img.dimensions();

    // Standard Minecraft cape texture is 64x32. Back of cape is at x=1..11, y=1..17 (10x16)
    let src_x = if cw >= 64 { 1 } else { 0 };
    let src_y = if ch >= 32 { 1 } else { 0 };
    let src_w = if cw >= 64 { 10 } else { cw };
    let src_h = if ch >= 32 { 16 } else { ch };

    let scale = 10;
    let mut high_res = ImageBuffer::from_pixel(src_w * scale, src_h * scale, Rgba([0, 0, 0, 0]));

    for y in 0..src_h {
        for x in 0..src_w {
            let px = cape_img.get_pixel(src_x + x, src_y + y);
            for sy in 0..scale {
                for sx in 0..scale {
                    high_res.put_pixel(x * scale + sx, y * scale + sy, *px);
                }
            }
        }
    }

    Ok(high_res)
}

/// Generates the official DV Crimson Cape image bytes and saves it if not already generated.
pub fn ensure_preset_capes() -> PathBuf {
    let capes_dir = get_capes_dir();
    let dv_cape_path = capes_dir.join("dv_crimson.png");

    if !dv_cape_path.exists() {
        // Create 64x32 standard cape
        let mut cape: RgbaImage = ImageBuffer::from_pixel(64, 32, Rgba([0, 0, 0, 0]));

        // Fill cape body with Obsidian Black & Crimson Red accents
        for y in 1..17 {
            for x in 1..11 {
                if y == 1 || y == 16 || x == 1 || x == 10 {
                    // Border: Crimson Red #FF3344
                    cape.put_pixel(x, y, Rgba([255, 51, 68, 255]));
                } else if (x == 5 || x == 6) && (5..=12).contains(&y) {
                    // Central DV Emblem Spine
                    cape.put_pixel(x, y, Rgba([255, 77, 94, 255]));
                } else if (y == 8 || y == 9) && (3..=8).contains(&x) {
                    // Central DV Crossbar
                    cape.put_pixel(x, y, Rgba([255, 51, 68, 255]));
                } else {
                    // Deep Obsidian Void #121318
                    cape.put_pixel(x, y, Rgba([18, 19, 24, 255]));
                }
            }
        }

        let _ = cape.save(&dv_cape_path);
    }

    dv_cape_path
}

/// Generates a default Steve skin if no skin is available.
pub fn ensure_default_steve_skin() -> PathBuf {
    let skins_dir = get_skins_dir();
    let steve_path = skins_dir.join("default_steve.png");

    if !steve_path.exists() {
        let mut skin: RgbaImage = ImageBuffer::from_pixel(64, 64, Rgba([0, 0, 0, 0]));

        // Head (8x8 at 8,8) - Classic skin tone and brown hair
        for y in 8..16 {
            for x in 8..16 {
                if y <= 10 {
                    // Hair
                    skin.put_pixel(x, y, Rgba([75, 45, 20, 255]));
                } else if y == 12 && (x == 10 || x == 13) {
                    // Eyes
                    skin.put_pixel(x, y, Rgba([40, 60, 160, 255]));
                } else {
                    // Skin
                    skin.put_pixel(x, y, Rgba([190, 130, 90, 255]));
                }
            }
        }

        // Torso (8x12 at 20,20) - Cyan shirt
        for y in 20..32 {
            for x in 20..28 {
                skin.put_pixel(x, y, Rgba([0, 160, 180, 255]));
            }
        }

        // Arms (4x12 at 44,20 and 36,52)
        for y in 20..32 {
            for x in 44..48 {
                if y < 24 {
                    skin.put_pixel(x, y, Rgba([0, 160, 180, 255]));
                } else {
                    skin.put_pixel(x, y, Rgba([190, 130, 90, 255]));
                }
            }
        }
        for y in 52..64 {
            for x in 36..40 {
                if y < 56 {
                    skin.put_pixel(x, y, Rgba([0, 160, 180, 255]));
                } else {
                    skin.put_pixel(x, y, Rgba([190, 130, 90, 255]));
                }
            }
        }

        // Legs (4x12 at 4,20 and 20,52) - Blue jeans
        for y in 20..32 {
            for x in 4..8 {
                skin.put_pixel(x, y, Rgba([40, 45, 110, 255]));
            }
        }
        for y in 52..64 {
            for x in 20..24 {
                skin.put_pixel(x, y, Rgba([40, 45, 110, 255]));
            }
        }

        let _ = skin.save(&steve_path);
    }

    steve_path
}

/// Fetch a player skin preview for display in the launcher.
/// Checks user-specific custom skin, universal skin, remote MC-Heads, and falls back to default Steve.
pub async fn get_or_fetch_player_skin(username: &str, is_microsoft: bool) -> PathBuf {
    let skins_dir = get_skins_dir();
    let preview_path = skins_dir.join(format!("{}_preview.png", username));
    let raw_custom_skin = skins_dir.join(format!("{}_raw.png", username));
    let universal_raw = get_universal_skin_path();
    let universal_preview = get_universal_skin_preview_path();

    // 1. If player-specific custom skin uploaded, use that
    if raw_custom_skin.exists() {
        if let Ok(preview) = render_skin_preview(&raw_custom_skin) {
            let _ = preview.save(&preview_path);
            return preview_path;
        }
    }

    // 2. If universal skin exists, display universally for every player
    if universal_raw.exists() {
        if universal_preview.exists()
            && fs::metadata(&universal_preview)
                .map(|m| m.len() > 0)
                .unwrap_or(false)
        {
            return universal_preview;
        }
        if let Ok(preview) = render_skin_preview(&universal_raw) {
            let _ = preview.save(&universal_preview);
            return universal_preview;
        }
    }

    // 3. If preview already exists and is not empty, use it
    if preview_path.exists()
        && fs::metadata(&preview_path)
            .map(|m| m.len() > 0)
            .unwrap_or(false)
    {
        return preview_path;
    }

    // 4. For Microsoft or recognized usernames, fetch 3D body render from MC-Heads
    if is_microsoft || (!username.is_empty() && username != "DVPlayer" && username != "Player") {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .build();

        if let Ok(c) = client {
            let url = format!("https://mc-heads.net/body/{}/right", username);
            if let Ok(resp) = c.get(&url).send().await {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes().await {
                        if !bytes.is_empty() {
                            let _ = fs::write(&preview_path, bytes);
                            return preview_path;
                        }
                    }
                }
            }
        }
    }

    // 5. Default Steve fallback
    let steve_skin = ensure_default_steve_skin();
    if let Ok(preview) = render_skin_preview(&steve_skin) {
        let _ = preview.save(&preview_path);
        return preview_path;
    }

    steve_skin
}

/// Fetch a player cape preview for display.
/// Checks user-specific custom cape, universal cape, or built-in presets.
pub fn get_or_create_cape_preview(username: &str, preset: Option<&str>) -> Option<PathBuf> {
    let capes_dir = get_capes_dir();
    let custom_cape = capes_dir.join(format!("{}_cape.png", username));
    let universal_cape = get_universal_cape_path();
    let preview_path = capes_dir.join(format!("{}_cape_preview.png", username));
    let universal_preview = get_universal_cape_preview_path();

    if custom_cape.exists() {
        if let Ok(preview) = render_cape_preview(&custom_cape) {
            let _ = preview.save(&preview_path);
            return Some(preview_path);
        }
    }

    if universal_cape.exists() {
        if universal_preview.exists()
            && fs::metadata(&universal_preview)
                .map(|m| m.len() > 0)
                .unwrap_or(false)
        {
            return Some(universal_preview);
        }
        if let Ok(preview) = render_cape_preview(&universal_cape) {
            let _ = preview.save(&universal_preview);
            return Some(universal_preview);
        }
    }

    if let Some(p) = preset {
        if p == "crimson" {
            let cape_file = ensure_preset_capes();
            if let Ok(preview) = render_cape_preview(&cape_file) {
                let _ = preview.save(&preview_path);
                return Some(preview_path);
            }
        }
    }

    None
}

/// Save a newly uploaded custom skin file for the specified username and optional GUID/UUID, applying universally.
pub fn save_custom_skin<P: AsRef<Path>>(
    username: &str,
    uuid: Option<&str>,
    src_file: P,
) -> Result<PathBuf, SkinError> {
    let skins_dir = get_skins_dir();
    let raw_dest = skins_dir.join(format!("{}_raw.png", username));
    let preview_dest = skins_dir.join(format!("{}_preview.png", username));
    let universal_raw = get_universal_skin_path();
    let universal_preview = get_universal_skin_preview_path();

    fs::copy(src_file.as_ref(), &raw_dest)?;
    let _ = fs::copy(src_file.as_ref(), &universal_raw);

    // Also persist by Account GUID / UUID if present
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = fs::copy(src_file.as_ref(), skins_dir.join(format!("{}_raw.png", clean)));
        }
    }

    // Verify and render preview
    let preview = render_skin_preview(&raw_dest)?;
    preview.save(&preview_dest)?;
    let _ = preview.save(&universal_preview);

    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = preview.save(skins_dir.join(format!("{}_preview.png", clean)));
        }
    }

    info!(
        "Saved custom skin for player {} (GUID: {:?}) and all profiles: {}",
        username,
        uuid,
        raw_dest.display()
    );
    Ok(preview_dest)
}

/// Save a newly uploaded custom cape file for the specified username and optional GUID/UUID, applying universally.
pub fn save_custom_cape<P: AsRef<Path>>(
    username: &str,
    uuid: Option<&str>,
    src_file: P,
) -> Result<PathBuf, SkinError> {
    let capes_dir = get_capes_dir();
    let raw_dest = capes_dir.join(format!("{}_cape.png", username));
    let preview_dest = capes_dir.join(format!("{}_cape_preview.png", username));
    let universal_raw = get_universal_cape_path();
    let universal_preview = get_universal_cape_preview_path();

    fs::copy(src_file.as_ref(), &raw_dest)?;
    let _ = fs::copy(src_file.as_ref(), &universal_raw);

    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = fs::copy(src_file.as_ref(), capes_dir.join(format!("{}_cape.png", clean)));
        }
    }

    let preview = render_cape_preview(&raw_dest)?;
    preview.save(&preview_dest)?;
    let _ = preview.save(&universal_preview);

    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = preview.save(capes_dir.join(format!("{}_cape_preview.png", clean)));
        }
    }

    info!(
        "Saved custom cape for player {} (GUID: {:?}) and all profiles: {}",
        username,
        uuid,
        raw_dest.display()
    );
    Ok(preview_dest)
}

/// Reset player skin back to default Steve/Alex.
pub fn reset_player_skin(username: &str, uuid: Option<&str>) {
    let skins_dir = get_skins_dir();
    let _ = fs::remove_file(skins_dir.join(format!("{}_raw.png", username)));
    let _ = fs::remove_file(skins_dir.join(format!("{}_preview.png", username)));
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = fs::remove_file(skins_dir.join(format!("{}_raw.png", clean)));
            let _ = fs::remove_file(skins_dir.join(format!("{}_preview.png", clean)));
        }
    }
    let _ = fs::remove_file(get_universal_skin_path());
    let _ = fs::remove_file(get_universal_skin_preview_path());
}

/// Reset player cape back to none or default.
pub fn reset_player_cape(username: &str, uuid: Option<&str>) {
    let capes_dir = get_capes_dir();
    let _ = fs::remove_file(capes_dir.join(format!("{}_cape.png", username)));
    let _ = fs::remove_file(capes_dir.join(format!("{}_cape_preview.png", username)));
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let _ = fs::remove_file(capes_dir.join(format!("{}_cape.png", clean)));
            let _ = fs::remove_file(capes_dir.join(format!("{}_cape_preview.png", clean)));
        }
    }
    let _ = fs::remove_file(get_universal_cape_path());
    let _ = fs::remove_file(get_universal_cape_preview_path());
}

/// Returns the path to the active raw skin file for the specified username/guid, or universal/default steve skin.
pub fn get_active_skin_path(username: Option<&str>, uuid: Option<&str>) -> PathBuf {
    let skins_dir = get_skins_dir();
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let user_skin = skins_dir.join(format!("{}_raw.png", clean));
            if user_skin.exists() {
                return user_skin;
            }
        }
    }
    if let Some(u) = username {
        let user_skin = skins_dir.join(format!("{}_raw.png", u));
        if user_skin.exists() {
            return user_skin;
        }
    }
    let universal_skin = get_universal_skin_path();
    if universal_skin.exists() {
        return universal_skin;
    }
    ensure_default_steve_skin()
}

/// Returns the path to the active cape file if present.
pub fn get_active_cape_path(username: Option<&str>, uuid: Option<&str>) -> Option<PathBuf> {
    let capes_dir = get_capes_dir();
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            let user_cape = capes_dir.join(format!("{}_cape.png", clean));
            if user_cape.exists() {
                return Some(user_cape);
            }
        }
    }
    if let Some(u) = username {
        let user_cape = capes_dir.join(format!("{}_cape.png", u));
        if user_cape.exists() {
            return Some(user_cape);
        }
    }
    let universal_cape = get_universal_cape_path();
    if universal_cape.exists() {
        return Some(universal_cape);
    }
    None
}

/// Computes the appropriate Minecraft resource pack format for pack.mcmeta.
fn compute_pack_format(mc_version: Option<&str>) -> u32 {
    let ver = match mc_version {
        Some(v) => v.trim(),
        None => return 34,
    };
    if ver.starts_with("1.6") || ver.starts_with("1.7") || ver.starts_with("1.8") {
        1
    } else if ver.starts_with("1.9") || ver.starts_with("1.10") {
        2
    } else if ver.starts_with("1.11") || ver.starts_with("1.12") {
        3
    } else if ver.starts_with("1.13") || ver.starts_with("1.14") {
        4
    } else if ver.starts_with("1.15") || ver == "1.16" || ver == "1.16.1" {
        5
    } else if ver.starts_with("1.16") {
        6
    } else if ver.starts_with("1.17") {
        7
    } else if ver.starts_with("1.18") {
        8
    } else if ver == "1.19" || ver == "1.19.1" || ver == "1.19.2" {
        9
    } else if ver == "1.19.3" {
        12
    } else if ver == "1.19.4" {
        13
    } else if ver == "1.20" || ver == "1.20.1" {
        15
    } else if ver == "1.20.2" {
        18
    } else if ver == "1.20.3" || ver == "1.20.4" {
        22
    } else if ver == "1.20.5" || ver == "1.20.6" {
        32
    } else if ver.starts_with("1.21.4") {
        46
    } else if ver.starts_with("1.21.2") || ver.starts_with("1.21.3") {
        42
    } else if ver.starts_with("1.21") {
        34
    } else {
        34
    }
}

/// Injects the Universal Player Skin & Cape into the Minecraft instance game directory.
/// Builds a native resource pack overriding all player model variants (both legacy 1.7-1.19 and modern 1.20+)
/// and automatically enables it in options.txt so every player sees their custom skin in-game.
pub fn ensure_in_game_skin_pack<P: AsRef<Path>>(
    game_dir: P,
    username: &str,
    uuid: Option<&str>,
    mc_version: Option<&str>,
) -> std::io::Result<()> {
    let game_dir = game_dir.as_ref();
    let rp_dir = game_dir.join("resourcepacks").join("DV_Universal_Skin");
    let _ = fs::create_dir_all(&rp_dir);

    // 1. Write pack.mcmeta with dynamic pack_format and valid supported_formats (capped at 64)
    let pack_fmt = compute_pack_format(mc_version);
    let mcmeta_path = rp_dir.join("pack.mcmeta");
    let mcmeta_content = format!(
        r#"{{
  "pack": {{
    "pack_format": {},
    "supported_formats": {{"min_inclusive": 1, "max_inclusive": 64}},
    "description": "Digital Vigital Universal Player Skin"
  }}
}}"#,
        pack_fmt
    );
    let _ = fs::write(mcmeta_path, mcmeta_content);

    // 2. Locate active skin file (player custom by UUID, username, or universal fallback)
    let active_skin_path = get_active_skin_path(Some(username), uuid);

    // 3. Copy skin to all player model texture paths (legacy and 1.20+ modern default characters)
    let entity_dir = rp_dir
        .join("assets")
        .join("minecraft")
        .join("textures")
        .join("entity");
    let player_dir = entity_dir.join("player");
    let wide_dir = player_dir.join("wide");
    let slim_dir = player_dir.join("slim");
    let _ = fs::create_dir_all(&entity_dir);
    let _ = fs::create_dir_all(&wide_dir);
    let _ = fs::create_dir_all(&slim_dir);

    // Legacy player textures (1.6-1.19)
    let _ = fs::copy(&active_skin_path, entity_dir.join("steve.png"));
    let _ = fs::copy(&active_skin_path, entity_dir.join("alex.png"));

    // Modern Minecraft 1.20+ all default player variants
    let default_models = [
        "steve", "alex", "ari", "efe", "kai", "makena", "noor", "sunny", "zuri",
    ];
    for name in &default_models {
        let _ = fs::copy(&active_skin_path, wide_dir.join(format!("{}.png", name)));
        let _ = fs::copy(&active_skin_path, slim_dir.join(format!("{}.png", name)));
    }

    // Ancient backward compatibility (<= 1.5.2) char.png (proper 64x32 layout for Alpha/Beta/1.0-1.5)
    let mob_dir = rp_dir
        .join("assets")
        .join("minecraft")
        .join("textures")
        .join("mob");
    let ancient_mob_dir = rp_dir.join("mob");
    let _ = fs::create_dir_all(&mob_dir);
    let _ = fs::create_dir_all(&ancient_mob_dir);
    let _ = fs::copy(&active_skin_path, entity_dir.join("char.png"));
    if let Ok(dyn_img) = image::open(&active_skin_path) {
        if dyn_img.height() == 64 {
            let legacy_32 = dyn_img.crop_imm(0, 0, 64, 32);
            let _ = legacy_32.save(mob_dir.join("char.png"));
            let _ = legacy_32.save(ancient_mob_dir.join("char.png"));
        } else {
            let _ = fs::copy(&active_skin_path, mob_dir.join("char.png"));
            let _ = fs::copy(&active_skin_path, ancient_mob_dir.join("char.png"));
        }
    } else {
        let _ = fs::copy(&active_skin_path, mob_dir.join("char.png"));
        let _ = fs::copy(&active_skin_path, ancient_mob_dir.join("char.png"));
    }

    // 4. Locate active cape file and write to entity/cape.png and player models
    let active_cape_path = get_active_cape_path(Some(username), uuid);
    let cape_entity_file = entity_dir.join("cape.png");
    if let Some(ref cp) = active_cape_path {
        let _ = fs::copy(cp, &cape_entity_file);
        let _ = fs::copy(cp, wide_dir.join("cape.png"));
        let _ = fs::copy(cp, slim_dir.join("cape.png"));
    } else if cape_entity_file.exists() {
        let _ = fs::remove_file(&cape_entity_file);
        let _ = fs::remove_file(wide_dir.join("cape.png"));
        let _ = fs::remove_file(slim_dir.join("cape.png"));
    }

    // 5. Populate offline skin mod caches for instant in-game resolution across loaders
    // Supports CybermindHUD, CustomSkinLoader, LocalSkin, OfflineSkin, and cachedImages
    let cybermind_skins = game_dir.join("cybermind-appearance-cache").join("skins");
    let cybermind_capes = game_dir.join("cybermind-appearance-cache").join("capes");
    let csl_skins = game_dir.join("CustomSkinLoader").join("LocalSkin").join("skins");
    let csl_capes = game_dir.join("CustomSkinLoader").join("LocalSkin").join("capes");
    let csl_direct_skins = game_dir.join("CustomSkinLoader").join("skins");
    let csl_direct_capes = game_dir.join("CustomSkinLoader").join("capes");
    let direct_skins = game_dir.join("skins");
    let cached_skins = game_dir.join("cachedImages").join("skins");
    let _ = fs::create_dir_all(&cybermind_skins);
    let _ = fs::create_dir_all(&cybermind_capes);
    let _ = fs::create_dir_all(&csl_skins);
    let _ = fs::create_dir_all(&csl_capes);
    let _ = fs::create_dir_all(&csl_direct_skins);
    let _ = fs::create_dir_all(&csl_direct_capes);
    let _ = fs::create_dir_all(&direct_skins);
    let _ = fs::create_dir_all(&cached_skins);

    // Compute standard Java Type 3 UUID (OfflinePlayer:<username>)
    let java_offline_uuid = {
        let mut digest = crate::accounts::minecraft_auth::md5_hash(format!("OfflinePlayer:{}", username).as_bytes());
        digest[6] = (digest[6] & 0x0f) | 0x30;
        digest[8] = (digest[8] & 0x3f) | 0x80;
        let hex_str = hex::encode(digest);
        format!(
            "{}-{}-{}-{}-{}",
            &hex_str[0..8],
            &hex_str[8..12],
            &hex_str[12..16],
            &hex_str[16..20],
            &hex_str[20..32]
        )
    };

    let mut id_keys = vec![
        username.to_string(),
        java_offline_uuid.clone(),
        java_offline_uuid.replace('-', ""),
    ];
    if let Some(guid) = uuid {
        let clean = guid.trim();
        if !clean.is_empty() {
            id_keys.push(clean.to_string());
            id_keys.push(clean.replace('-', ""));
        }
    }

    for key in &id_keys {
        let _ = fs::copy(&active_skin_path, cybermind_skins.join(format!("{}.png", key)));
        let _ = fs::copy(&active_skin_path, cybermind_skins.join(format!("{}-slim.png", key)));
        let _ = fs::copy(&active_skin_path, csl_skins.join(format!("{}.png", key)));
        let _ = fs::copy(&active_skin_path, direct_skins.join(format!("{}.png", key)));
        let _ = fs::copy(&active_skin_path, cached_skins.join(format!("{}.png", key)));

        if let Some(ref cp) = active_cape_path {
            let _ = fs::copy(cp, cybermind_capes.join(format!("{}.png", key)));
            let _ = fs::copy(cp, csl_capes.join(format!("{}.png", key)));
        }
    }

    // 6. Ensure options.txt automatically enables DV_Universal_Skin using clean modern key
    let options_path = game_dir.join("options.txt");
    let modern_entry = "file/DV_Universal_Skin";
    if options_path.exists() {
        if let Ok(content) = fs::read_to_string(&options_path) {
            let mut new_lines = Vec::new();
            let mut replaced = false;
            for line in content.lines() {
                if line.starts_with("resourcePacks:") {
                    let mut list = line.to_string();
                    // Strip any malformed legacy un-prefixed entry
                    list = list.replace("\"DV_Universal_Skin\",", "")
                               .replace(",\"DV_Universal_Skin\"", "")
                               .replace("\"DV_Universal_Skin\"", "");
                    if !list.contains("\"file/DV_Universal_Skin\"") {
                        if let Some(end_bracket) = list.rfind(']') {
                            let before = list[..end_bracket].trim();
                            let updated = if before.ends_with('[') {
                                format!("{}[\"{}\"]", &before[..before.len() - 1], modern_entry)
                            } else {
                                format!("{},\"{}\"]", before, modern_entry)
                            };
                            list = updated;
                        }
                    }
                    new_lines.push(list);
                    replaced = true;
                    continue;
                }
                if line.starts_with("incompatibleResourcePacks:") {
                    let mut list = line.to_string();
                    list = list.replace("\"file/DV_Universal_Skin\",", "")
                               .replace(",\"file/DV_Universal_Skin\"", "")
                               .replace("\"file/DV_Universal_Skin\"", "")
                               .replace("\"DV_Universal_Skin\",", "")
                               .replace(",\"DV_Universal_Skin\"", "")
                               .replace("\"DV_Universal_Skin\"", "");
                    new_lines.push(list);
                    continue;
                }
                new_lines.push(line.to_string());
            }
            if !replaced {
                new_lines.push(format!("resourcePacks:[\"vanilla\",\"{}\"]", modern_entry));
            }
            let _ = fs::write(&options_path, new_lines.join("\n"));
        }
    } else {
        let default_options = format!(
            "resourcePacks:[\"vanilla\",\"{}\"]\nincompatibleResourcePacks:[]\n",
            modern_entry
        );
        let _ = fs::write(&options_path, default_options);
    }

    info!(
        "Universal in-game player skin & cape pack synchronized for {} (GUID: {:?}) in {}",
        username,
        uuid,
        rp_dir.display()
    );
    Ok(())
}

/// Synchronizes the universal skin & cape resource pack to all instances in `instances_dir`.
pub fn sync_universal_skin_to_instances<P: AsRef<Path>>(
    instances_dir: P,
    username: &str,
    uuid: Option<&str>,
) {
    if let Ok(entries) = fs::read_dir(instances_dir.as_ref()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let game_dir = path.join(".minecraft");
                if game_dir.exists() || path.join("instance.json").exists() {
                    let mc_ver = if let Ok(data) = fs::read_to_string(path.join("instance.json")) {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&data) {
                            val.get("minecraft_version").and_then(|v| v.as_str()).map(|s| s.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let _ = ensure_in_game_skin_pack(&game_dir, username, uuid, mc_ver.as_deref());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_default_steve_skin() {
        let steve = ensure_default_steve_skin();
        assert!(steve.exists());
        let img = image::open(&steve).expect("Steve skin should open");
        assert_eq!(img.width(), 64);
        assert_eq!(img.height(), 64);
    }

    #[test]
    fn test_render_skin_preview() {
        let steve = ensure_default_steve_skin();
        let preview = render_skin_preview(&steve).expect("Preview should render");
        assert_eq!(preview.width(), 160);
        assert_eq!(preview.height(), 320);
    }

    #[test]
    fn test_ensure_preset_cape() {
        let cape = ensure_preset_capes();
        assert!(cape.exists());
        let img = image::open(&cape).expect("DV Cape should open");
        assert_eq!(img.width(), 64);
        assert_eq!(img.height(), 32);

        let preview = render_cape_preview(&cape).expect("Cape preview should render");
        assert_eq!(preview.width(), 100);
        assert_eq!(preview.height(), 160);
    }

    #[test]
    fn test_invalid_skin_dimensions() {
        let temp_dir = tempfile::tempdir().unwrap();
        let invalid_path = temp_dir.path().join("invalid.png");
        let invalid_img: RgbaImage = ImageBuffer::from_pixel(30, 30, Rgba([255, 0, 0, 255]));
        invalid_img.save(&invalid_path).unwrap();

        let result = render_skin_preview(&invalid_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_pack_format_versions() {
        assert_eq!(compute_pack_format(Some("1.8.9")), 1);
        assert_eq!(compute_pack_format(Some("1.12.2")), 3);
        assert_eq!(compute_pack_format(Some("1.16.5")), 6);
        assert_eq!(compute_pack_format(Some("1.20.1")), 15);
        assert_eq!(compute_pack_format(Some("1.21.1")), 34);
        assert_eq!(compute_pack_format(Some("1.21.4")), 46);
        assert_eq!(compute_pack_format(None), 34);
    }

    #[test]
    fn test_ensure_in_game_skin_pack_generation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let game_dir = temp_dir.path();
        let res = ensure_in_game_skin_pack(game_dir, "TestPlayer", Some("guid-12345"), Some("1.21.1"));
        assert!(res.is_ok());

        let rp_dir = game_dir.join("resourcepacks").join("DV_Universal_Skin");
        assert!(rp_dir.join("pack.mcmeta").exists());
        assert!(rp_dir.join("assets").join("minecraft").join("textures").join("entity").join("steve.png").exists());
        assert!(rp_dir.join("assets").join("minecraft").join("textures").join("entity").join("char.png").exists());

        // Mod caches
        let csl_skins = game_dir.join("CustomSkinLoader").join("LocalSkin").join("skins");
        assert!(csl_skins.join("TestPlayer.png").exists());
        assert!(csl_skins.join("guid-12345.png").exists());

        let cm_skins = game_dir.join("cybermind-appearance-cache").join("skins");
        assert!(cm_skins.join("TestPlayer.png").exists());
        assert!(cm_skins.join("guid-12345.png").exists());

        // Options.txt
        let options_file = game_dir.join("options.txt");
        assert!(options_file.exists());
        let content = fs::read_to_string(options_file).unwrap();
        assert!(content.contains("DV_Universal_Skin"));
        assert!(content.contains("file/DV_Universal_Skin"));
    }
}
