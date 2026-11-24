# Digital Vigital (DV) Launcher — Release Notes v1.0.2

**Release Date:** November 24, 2026  
**Repository:** [https://github.com/t041n/DV-Client](https://github.com/t041n/DV-Client)  
**Project Owner:** **_tor.in** ([@t041n](https://github.com/t041n))  
**Target:** Windows x64 (Windows 10 / 11)  

---

## 🌟 What's New in v1.0.2

Digital Vigital v1.0.2 delivers built-in default Fabric mod integration with automatic Fabric API resolution, complete singleplayer world save manager fixes and importers, static engine linkage eliminating external DLL issues, and an uncluttered user interface.

---

### 🧩 Default Custom Fabric Plugin & Auto Fabric API
- **Built-in DV Client In-Game Plugin (`dvclient-fabric-1.0.0.jar`)**:
  - Embedded directly into the launcher core engine and distribution packages.
  - Automatically installed into any Fabric instance upon creation or launch—regardless of the Minecraft version chosen.
  - Delivers in-game DV Client HUD overlays, auto-reconnect, and performance optimizations out of the box.
- **Universal Fabric API Auto-Resolution**:
  - Automatically detects if Fabric API is missing from the instance's mods folder.
  - Queries and downloads the exact matching `fabric-api` version from Modrinth automatically during instance creation and before launch.

---

### 🗺️ World Saves & Backup System Fixes
- **Directory Opener Fix**:
  - Fixed the PowerShell `ExitStatus(1)` directory error by safely creating the target saves folder before launching Windows File Explorer.
- **Multi-Path World Scanning**:
  - Supports scanning worlds from both `.minecraft/saves` and root `saves` directories with folder deduplication.
- **1-Click World Importers**:
  - **Import .zip**: Added native file picker to extract and install zipped world backups instantly.
  - **Import Vanilla**: Added 1-click import to copy existing singleplayer worlds directly from Vanilla `%APPDATA%/.minecraft/saves`.

---

### ⚡ Core Engine Static Linkage
- **Eliminated DLL Dependencies**:
  - Statically linked `dv_core` directly into the launcher executable, resolving the *"Could not load 'dv_core.dll'"* startup alert.
  - Standalone executable now runs out of the box with zero runtime configuration.

---

### 🧹 Clean & Focused UI
- **Removed Server Ping Section**:
  - Removed the multiplayer quick-connect card and add-server modal to keep the Launch & Play dashboard focused and lightweight.
- **Removed Essential FPS Suite Prompts**:
  - Streamlined the Mod Studio and Quick Feature grid by removing redundant FPS boost suite cards.
- **Balanced 3-Column Dashboard**:
  - Reorganized feature cards into Storage Management, World Saves & Backups, and In-Game Screenshot Gallery.

---

## 📦 Downloads & Available Packages

| Package | Size | Description |
| :--- | :---: | :--- |
| [**`DigitalVigital-Setup-1.0.2.exe`**](https://github.com/t041n/DV-Client/releases/download/v1.0.2/DigitalVigital-Setup-1.0.2.exe) | **25.6 MB** | **Windows Setup Installer**: The official NSIS installer with custom drive selection (C:, D:, E:, etc.), Start Menu & Desktop shortcuts, clean uninstaller, bundled WebView2Loader, and embedded Fabric plugin. |
| [**`DigitalVigital-v1.0.2-windows-x64-portable.zip`**](https://github.com/t041n/DV-Client/releases/download/v1.0.2/DigitalVigital-v1.0.2-windows-x64-portable.zip) | **49.1 MB** | **Portable Standalone Package**: Contains `dv-launcher-app.exe`, `WebView2Loader.dll`, bundled Fabric plugin, assets, and documentation. No installation required. |
| [**`dv-launcher-app.exe`**](https://github.com/t041n/DV-Client/releases/download/v1.0.2/dv-launcher-app.exe) | **70.5 MB** | **Direct Standalone Executable**: Self-contained Windows executable with core engine and in-game plugin embedded. |

---

## 🔒 SHA-256 Checksums

```text
9FC9488FAECD3732004E691DDDF4740086928D3808F709D7ADA4ECE495C59485  DigitalVigital-Setup-1.0.2.exe
F8051862119CF4C5A70F49CC0DC0067DAF1F1FEA9E9C4CD1CD1964D611450982  DigitalVigital-v1.0.2-windows-x64-portable.zip
E229ADB7C5CE4EAFF367FF13CF524596E86E3A996117F94C9C1956D0358627C3  dv-launcher-app.exe
```

---

## 🚀 Getting Started

1. **Setup Installer**: Run `DigitalVigital-Setup-1.0.2.exe` to install to any drive or partition.
2. **Portable Version**: Extract `DigitalVigital-v1.0.2-windows-x64-portable.zip` anywhere and launch `dv-launcher-app.exe`.
