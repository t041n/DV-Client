# DV Client (Digital Vigital) — Android Architecture & Roadmap

This document outlines the architectural blueprint, native bridges, and implementation milestones for porting **Digital Vigital (DV) Client** to Android devices as a fully playable Minecraft: Java Edition client.

---

## 1. High-Level System Architecture

```
+---------------------------------------------------------------------------------+
|                       DV Client UI (Tauri v2 Mobile Shell)                     |
|            Svelte 5 + Tailwind CSS + Responsive Touch Layout + Dark Theme       |
+---------------------------------------------------------------------------------+
                                       │
                         Tauri IPC / JNI FFI Bridge
                                       │
                                       ▼
+---------------------------------------------------------------------------------+
|                         DV Core Engine (Rust Backend)                           |
|      - Profile / Instance Management        - Modrinth / Catalog Downloader    |
|      - Universal Skin Synchronizer           - Hardware / GPU Capability Detector|
|      - Minecraft Asset & Library Verifier   - Crash & Diagnostic Engine         |
+---------------------------------------------------------------------------------+
                                       │
                  Native Android Execution Supervisor (JNI)
                                       │
     ┌─────────────────────────────────┴─────────────────────────────────┐
     ▼                                                                   ▼
+──────────────────────────+                               +──────────────────────────+
|   Embedded OpenJDK ARM64 |                               | Graphics Translation     |
|   (Java 8 / 17 / 21)     |                               | (Zink / Turnip / GL4ES)  |
| - Android Bionic runtime |                               | - Desktop OpenGL 3.2+    |
| - Proot/musl isolation   |                               |   to Mobile Vulkan/GLES  |
+──────────────────────────+                               +──────────────────────────+
     │                                                                   │
     └─────────────────────────────────┬─────────────────────────────────┘
                                       ▼
+---------------------------------------------------------------------------------+
|                           Minecraft: Java Edition (LWJGL3)                      |
|      - Intercepted GLFW window calls redirected to Android ANativeWindow        |
|      - Custom SurfaceView rendering target                                     |
|      - Virtual On-Screen Controller Overlay (Joystick, Touch Gestures, Hotbar) |
+---------------------------------------------------------------------------------+
```

---

## 2. Technical Pillars & Problem Breakdown

### A. JVM Execution on Android (ARM64 OpenJDK)
* **Problem:** Android uses the Dalvik/ART runtime, not a standard OpenJDK Java Virtual Machine. Standard `std::process::Command` cannot spawn `javaw` on Android.
* **Solution:** 
  - Embed and dynamically extract a lightweight, Android Bionic-compatible OpenJDK (Java 8 for Minecraft < 1.17, Java 17 for 1.17–1.20.4, Java 21 for 1.20.5+).
  - Use a native JNI bridge (`libjvm.so`) or an isolated process runner (`execve` within application sandbox) to launch Knot (Fabric), NeoForge, Forge, or Vanilla entrypoints.

### B. Graphics Subsystem Translation
* **Problem:** Minecraft: Java Edition renders using desktop **OpenGL 3.2+ Core Profile**. Android GPUs (Qualcomm Adreno, ARM Mali, PowerVR) only support **OpenGL ES 3.2** and **Vulkan**.
* **Solution:**
  - **Zink (Mesa Vulkan Driver):** Runs desktop OpenGL on top of native mobile Vulkan. Provides the highest fidelity and modern Minecraft 1.20+ compatibility.
  - **Turnip (Open-source Adreno Vulkan driver):** Replaces buggy proprietary Qualcomm drivers on Snapdragon chipsets for significant FPS gains.
  - **GL4ES:** Lightweight fallback translation to OpenGL ES 2.0/3.0 for older legacy devices and older Minecraft versions (< 1.17).

### C. Windowing & LWJGL3 Redirection
* **Problem:** Minecraft relies on GLFW/X11 to create windows and receive keyboard/mouse events.
* **Solution:**
  - Build custom JNI bindings for `libglfw` and `liblwjgl` that route window creation to an Android `SurfaceView` / `ANativeWindow`.
  - Handle surface resize, orientation changes (locked to landscape), and display refresh rate matching (60Hz / 90Hz / 120Hz).

### D. Input Subsystem (Virtual Touch Controls & Gamepad)
* **Problem:** Minecraft Java expects a physical keyboard and mouse.
* **Solution:**
  - High-performance transparent on-screen virtual touch controls:
    - Left virtual analog joystick / D-pad (movement, double-tap sprint).
    - Right touch area for 360° camera look and drag-to-aim.
    - Configurable on-screen action buttons: Attack/Destroy, Place/Use, Jump, Sneak, Inventory, F3 (Debug), Chat.
    - Quick-scroll bottom hotbar.
  - Native Bluetooth/OTG physical controller and USB keyboard/mouse support.

---

## 3. Implementation Phases & Milestones

### Phase 1: Tauri v2 Android Initialization & Touch UI
- [ ] Initialize Tauri Android project structure (`gen/android`).
- [ ] Configure `Cargo.toml` with `crate-type = ["staticlib", "cdylib", "rlib"]`.
- [ ] Optimize Svelte frontend components for mobile landscape touch screens (responsive layout, larger hit targets, pull-to-refresh).
- [ ] Configure Android manifest permissions: `INTERNET`, `VIBRATE`, `ACCESS_NETWORK_STATE`, and storage access.

### Phase 2: OpenJDK Runtime Management
- [ ] Create runtime downloader in Rust for Android ARM64 OpenJDK (8, 17, 21) builds.
- [ ] Implement scoped application directory extraction (`/data/data/fun.digitalvigital.launcher/runtimes`).
- [ ] Verify classpath builder and duplicate ASM class deduplication for ARM64 jars.

### Phase 3: Pojav-Compatible Native Engine Bridge
- [ ] Implement JNI launcher layer (`libdv_mobile_exec.so`).
- [ ] Integrate OpenGL-to-Vulkan translation pipeline (Zink / VirGL / GL4ES).
- [ ] Implement LWJGL3 native window redirection onto Android `SurfaceView`.

### Phase 4: Virtual Controls & In-Game Input Overlay
- [ ] Build customizable on-screen touch control overlay (JSON layout persistence).
- [ ] Add touch camera acceleration, sensitivity sliders, and haptic feedback.
- [ ] Support physical Bluetooth controllers (Xbox, DualShock, generic HID).

### Phase 5: Verification & Packaging
- [ ] Build signed release APK and Android App Bundle (`.aab`) for testing.
- [ ] Test across different SoC architectures: Snapdragon (Adreno), MediaTek (Mali), Google Tensor (Mali).
- [ ] Benchmark FPS with Sodium / Iris / Lithium optimization suite.
