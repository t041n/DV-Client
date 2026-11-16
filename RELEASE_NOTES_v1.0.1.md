# Digital Vigital (DV) Launcher — Release Notes v1.0.1

**Release Date:** November 16, 2026  
**Repository:** [https://github.com/t041n/DV-Client](https://github.com/t041n/DV-Client)  
**Project Owner:** **_tor.in** ([@t041n](https://github.com/t041n))  
**Target:** Windows x64 (Windows 10 / 11)  

---

## 🌟 What's New in v1.0.1

Digital Vigital v1.0.1 brings critical fixes for Minecraft skin rendering, Fabric classpath stability, crash diagnostic accuracy, UI overlap refinements, and updates the official project ownership to **_tor.in**.

---

### 🎨 In-Game Skin Studio & SMP Multiplayer Visibility
- **Universal Skin Synchronization Engine**:
  - Automatically compiles and synchronizes your selected skin into the `DV_Universal_Skin` resource pack across all instances.
  - Automatically sanitizes `options.txt` on modern Minecraft versions (1.20+ / 1.21+), purging `DV_Universal_Skin` from `incompatibleResourcePacks:` so Minecraft will never block or disable your custom skin pack.
- **1-Click "Force Sync"**:
  - Added a dedicated **Force Sync** button directly on the Universal In-Game Skin Sync card in the Skin Studio. One click instantly forces synchronization into the instance folders and mod caches.
- **SMP & Multiplayer Skin Visibility Controls**:
  - **1-Click CustomSkinLoader Integration**: Easily install `CustomSkinLoader` via Modrinth to guarantee skin rendering across multiplayer servers and offline SMPs.
  - **In-Game Command Helper**: Added a 1-click **Copy Command** (`/skin <player>`) for servers running SkinsRestorer or SkinSystem.
- **Refined Studio & 3D Stage**:
  - Cleaned up cosmetic collections and optimized the 3D skin canvas for fluid 60 FPS rotation and animation preview.

---

### 🛠️ Stability & Crash Diagnostics
- **Duplicate ASM Classpath Crash Fix**:
  - Fixed a fatal startup crash on Fabric 1.21+ caused by multiple versions of ASM libraries (`asm-9.10.1.jar` and `asm-9.6.jar`):
    ```text
    Caused by: java.lang.IllegalStateException: duplicate ASM classes found on classpath:
    .../asm-9.10.1.jar!/.../ClassReader.class, .../asm-9.6.jar!/.../ClassReader.class
    ```
  - Added smart classpath deduplication in the launcher JVM argument builder to filter out secondary conflicting ASM libraries before spawning Minecraft.
- **Diagnostic Engine Hardening**:
  - Fixed a bug where early launch failures and uncaught JVM initialization errors were erroneously categorized as "Session Finished".
  - Startup initialization errors and bytecode conflicts are now accurately flagged with actionable fix suggestions and direct links to the Live Console.

---

### 🖥️ UI & Layout Polish
- **Alert Banner Layout Fix**:
  - Fixed banner and button overlap in the crash notification bar on compact and laptop resolutions.
  - Added a dedicated, styled dismiss button ("Dismiss alert") to dismiss post-game notifications.
- **Owner & Developer Profile**:
  - Updated the About page showcase to feature **_tor.in** as the Project Owner and Lead Architect.
  - Direct links to GitHub profile ([https://github.com/t041n](https://github.com/t041n)) and new project repository.

---

## 📦 Downloads & Available Packages

| Package | Size | Description |
| :--- | :---: | :--- |
| [**`DigitalVigital-Setup-1.0.1.exe`**](https://github.com/t041n/DV-Client/releases/download/v1.0.1/DigitalVigital-Setup-1.0.1.exe) | **~12 MB** | **Windows Setup Installer**: Authenticode-signed installer with custom drive selection, Start Menu & Desktop shortcuts, clean uninstaller, bundled WebView2Loader, and Add/Remove Programs registration. |
| [**`DigitalVigital-v1.0.1-windows-x64-portable.zip`**](https://github.com/t041n/DV-Client/releases/download/v1.0.1/DigitalVigital-v1.0.1-windows-x64-portable.zip) | **~15 MB** | **Portable Standalone Package**: Contains signed `DigitalVigitalLauncher.exe`, `dv_core.dll`, `WebView2Loader.dll`, `DigitalVigital.cer`, `Trust-DigitalVigital.bat`, assets, and documentation. No installation required. |
| [**`DigitalVigitalLauncher.exe`**](https://github.com/t041n/DV-Client/releases/download/v1.0.1/DigitalVigitalLauncher.exe) | **1.4 MB** | **Direct Executable**: Standalone Windows executable with embedded primary icon resource (`1 ICON`) and Authenticode signature. |
| [**`dv_core.dll`**](https://github.com/t041n/DV-Client/releases/download/v1.0.1/dv_core.dll) | **42 MB** | **Modular Rust Core Engine**: High-performance release DLL powering the launcher runtime. |

---

## 🔒 SHA-256 Checksums

```text
1fd03ce00dc4c785b3d4d9fd5f6a6d667c26940069ab0ca515de35c6f7874d3a  DigitalVigital-Setup-1.0.1.exe
e0ae4f531fb256923ea5828e0654080cca348ef0e01e6477e81d7eb5c7c48c2d  DigitalVigital-v1.0.1-windows-x64-portable.zip
62f584fdda32846c018390dc17bf5cfb2218b1042cd2265fe2650fe7dd81712e  DigitalVigitalLauncher.exe
ae3ab2896bfa05e878b353e31bf3a09a6026bf0eeafafdc8f5665aa4d21f9f7d  dv_core.dll
```

---

## 🚀 Getting Started

1. **Setup Installer**: Run `DigitalVigital-Setup-1.0.1.exe` and follow the on-screen steps.
2. **Portable Version**: Extract `DigitalVigital-v1.0.1-windows-x64-portable.zip` anywhere on your computer and launch `DigitalVigitalLauncher.exe`.
3. *(Optional for testers)*: If Windows SmartScreen prompts on first run, right-click `Trust-DigitalVigital.bat` and select **Run as Administrator** to trust the certificate on your PC.
