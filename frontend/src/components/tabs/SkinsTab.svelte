<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { launcher } from "../../state/launcher.svelte";
  import * as skinview3d from "skinview3d";
  import { DirectionalLight } from "three";
  import {
    type SkinModelType,
    type SkinAnimationType,
  } from "../../constants/skins";
  import {
    User,
    Upload,
    Shield,
    RotateCcw,
    Layers,
    Sparkles,
    Trash2,
    Download,
    Play,
    Pause,
    Palette,
    Check,
    Loader2,
    Sparkle,
    Shirt,
    ArrowUpRight,
    Copy,
    Fingerprint,
    Info,
    Globe,
    RefreshCw,
    CheckCircle2,
    Zap,
  } from "lucide-svelte";

  let canvasEl: HTMLCanvasElement | null = $state(null);
  let viewer: skinview3d.SkinViewer | null = $state(null);

  let animationMode = $state<SkinAnimationType>("walk");
  let animationSpeed = $state(0.8);
  let isPaused = $state(false);
  let isAutoRotating = $state(false);
  let modelType = $state<SkinModelType>("classic");

  let isDraggingFile = $state(false);

  function copyGuidToClipboard() {
    if (!launcher.activeAccount?.id) return;
    navigator.clipboard.writeText(launcher.activeAccount.id);
    launcher.showToast(`Account GUID copied: ${launcher.activeAccount.id}`);
  }

  function initViewer() {
    if (!canvasEl) return;
    if (viewer) {
      viewer.dispose();
      viewer = null;
    }

    try {
      const skinSource = launcher.activeSkinDataUrl;
      const model = modelType === "slim" ? "slim" : "default";

      viewer = new skinview3d.SkinViewer({
        canvas: canvasEl,
        width: 320,
        height: 440,
        skin: skinSource,
        model: model,
      });

      viewer.fov = 70;
      viewer.zoom = 0.92;

      // Studio-grade balanced lighting: Bright, crisp, and vibrant without dark muddiness
      viewer.globalLight.intensity = 2.4;
      viewer.cameraLight.intensity = 1.8;

      // Key light: crisp highlights from top-right front
      const keyLight = new DirectionalLight(0xffffff, 1.3);
      keyLight.position.set(20, 50, 40);
      viewer.scene.add(keyLight);

      // Fill light: soft cool fill to illuminate left and lower shadow zones
      const fillLight = new DirectionalLight(0xedf2ff, 0.9);
      fillLight.position.set(-20, 20, 30);
      viewer.scene.add(fillLight);

      // Rim light: warm back-light to outline character shoulders and hair edges
      const rimLight = new DirectionalLight(0xffecd2, 0.8);
      rimLight.position.set(0, 30, -40);
      viewer.scene.add(rimLight);

      applyAnimation(animationMode);
    } catch (e) {
      console.warn("Could not initialize 3D skin viewer:", e);
    }
  }

  function applyAnimation(mode: SkinAnimationType) {
    if (!viewer) return;
    animationMode = mode;
    isPaused = false;

    if (mode === "walk") {
      const anim = new skinview3d.WalkingAnimation();
      anim.speed = animationSpeed;
      viewer.animation = anim;
    } else if (mode === "run") {
      const anim = new skinview3d.RunningAnimation();
      anim.speed = animationSpeed * 1.2;
      viewer.animation = anim;
    } else if (mode === "fly") {
      const anim = new skinview3d.FlyingAnimation();
      anim.speed = animationSpeed;
      viewer.animation = anim;
    } else {
      viewer.animation = null;
    }
  }

  function togglePause() {
    if (!viewer || !viewer.animation) return;
    isPaused = !isPaused;
    viewer.animation.paused = isPaused;
  }

  function toggleAutoRotate() {
    if (!viewer) return;
    isAutoRotating = !isAutoRotating;
    viewer.autoRotate = isAutoRotating;
    viewer.autoRotateSpeed = 1.2;
  }

  function setCameraAngle(angleRad: number) {
    if (!viewer) return;
    viewer.playerObject.rotation.y = angleRad;
    viewer.camera.position.set(0, 0, 42);
    viewer.camera.lookAt(0, 0, 0);
  }

  function setModelType(type: SkinModelType) {
    modelType = type;
    launcher.skinModelType = type;
    if (viewer) {
      viewer.loadSkin(launcher.activeSkinDataUrl, {
        model: type === "slim" ? "slim" : "default",
      });
    }
  }

  // Reactively update 3D skin when launcher state changes
  $effect(() => {
    const skin = launcher.activeSkinDataUrl;
    if (viewer && skin) {
      viewer.loadSkin(skin, {
        model: modelType === "slim" ? "slim" : "default",
      });
    }
  });

  onMount(() => {
    initViewer();
    if (!launcher.activeSkinDataUrl) {
      launcher.loadPlayerSkin();
    }
  });

  onDestroy(() => {
    if (viewer) {
      viewer.dispose();
      viewer = null;
    }
  });

  // Handle Drag & Drop of skin/cape image files directly onto canvas
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDraggingFile = true;
  }

  function handleDragLeave() {
    isDraggingFile = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDraggingFile = false;
    if (!e.dataTransfer || !e.dataTransfer.files.length) return;

    const file = e.dataTransfer.files[0];
    if (!file.name.toLowerCase().endsWith(".png")) {
      launcher.showToast("Please drop a valid .PNG skin or cape file.");
      return;
    }

    const reader = new FileReader();
    reader.onload = async () => {
      const dataUrl = reader.result as string;
      await launcher.applySkinBase64(dataUrl);
    };
    reader.readAsDataURL(file);
  }

  function downloadCurrentSkin() {
    if (!launcher.activeSkinDataUrl) return;
    const a = document.createElement("a");
    a.href = launcher.activeSkinDataUrl;
    a.download = `${launcher.activeAccount?.username || "minecraft"}_skin.png`;
    a.click();
    launcher.showToast("Skin texture downloaded!");
  }
</script>

<div class="h-full overflow-y-auto p-4 sm:p-6 lg:p-8 space-y-5 sm:space-y-6">
  <!-- Top Bar & Profile Header -->
  <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-white/8">
    <div>
      <h2 class="text-2xl font-black text-white uppercase tracking-tight flex items-center gap-3">
        <div class="w-8 h-8 rounded-xl bg-red-500/10 border border-red-500/20 flex items-center justify-center text-red-400">
          <Shirt class="w-4 h-4" />
        </div>
        <span>Minecraft Skin Studio</span>
      </h2>
      <p class="text-xs text-neutral-300 mt-1 font-medium">
        Customize player skins with real-time 3D rendering and in-game backward compatibility across all versions.
      </p>
    </div>

    <!-- Active Player & GUID Badge -->
    {#if launcher.activeAccount}
      <div class="flex flex-col sm:flex-row sm:items-center gap-3 px-4 py-2.5 rounded-2xl bg-white/4 border border-white/8 shadow-md">
        <div class="flex items-center gap-3">
          <div class="w-9 h-9 rounded-xl bg-red-500/15 border border-red-500/30 flex items-center justify-center text-red-300 font-black text-xs shrink-0">
            {launcher.activeAccount.username.substring(0, 2).toUpperCase()}
          </div>
          <div>
            <div class="text-xs font-bold text-white flex items-center gap-1.5">
              <span>{launcher.activeAccount.username}</span>
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
            </div>
            <div class="text-[10px] text-neutral-400 uppercase tracking-wider font-semibold">
              {launcher.activeAccount.account_type === "microsoft" ? "Microsoft" : "Offline"} Profile
            </div>
          </div>
        </div>

        <div class="hidden sm:block h-6 w-px bg-white/10"></div>

        <!-- GUID / UUID Badge with Quick Copy -->
        <div class="flex items-center gap-2 bg-black/40 px-2.5 py-1.5 rounded-xl border border-white/6">
          <Fingerprint class="w-3.5 h-3.5 text-red-400 shrink-0" />
          <div class="flex flex-col">
            <span class="text-[9px] text-neutral-400 uppercase tracking-widest font-mono font-bold">Account GUID</span>
            <span class="text-[11px] text-neutral-200 font-mono font-semibold truncate max-w-[150px] md:max-w-[190px]">
              {launcher.activeAccount.id}
            </span>
          </div>
          <button
            type="button"
            onclick={copyGuidToClipboard}
            class="p-1 rounded-lg hover:bg-white/10 text-neutral-400 hover:text-white transition-colors cursor-pointer ml-1"
            title="Copy Account GUID / UUID"
          >
            <Copy class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    {/if}
  </div>

  <!-- Main Grid Layout -->
  <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
    <!-- Left: 3D Stage & Rig Controls (5 Cols) -->
    <div class="lg:col-span-5 flex flex-col items-center">
      <div
        ondragover={handleDragOver}
        ondragleave={handleDragLeave}
        ondrop={handleDrop}
        role="region"
        aria-label="3D Skin Viewer Stage"
        class="relative rounded-3xl overflow-hidden border {isDraggingFile
          ? 'border-red-500 ring-2 ring-red-500/50 bg-red-500/10'
          : 'border-white/10 bg-gradient-to-b from-[#141724]/90 via-[#0d0e17] to-[#07080d]'} p-6 shadow-2xl flex flex-col items-center w-full transition-all"
      >
        <!-- Atmospheric Studio Lighting Ring -->
        <div class="absolute -top-16 -left-16 w-52 h-52 bg-red-600/15 rounded-full blur-3xl pointer-events-none"></div>
        <div class="absolute -bottom-16 -right-16 w-52 h-52 bg-cyan-600/10 rounded-full blur-3xl pointer-events-none"></div>

        <!-- 3D Canvas Stage -->
        <div class="relative w-full flex items-center justify-center">
          <canvas
            bind:this={canvasEl}
            class="rounded-2xl cursor-grab active:cursor-grabbing shadow-2xl select-none"
          ></canvas>

          {#if launcher.isLoadingSkin}
            <div class="absolute inset-0 bg-black/70 backdrop-blur-sm rounded-2xl flex flex-col items-center justify-center gap-2.5 text-xs text-white">
              <Loader2 class="w-7 h-7 text-red-500 animate-spin" />
              <span class="font-bold tracking-wide">Compiling 3D Textures...</span>
            </div>
          {/if}

          <!-- Drag & Drop Overlay Prompt -->
          {#if isDraggingFile}
            <div class="absolute inset-0 bg-red-950/80 backdrop-blur-sm rounded-2xl border-2 border-dashed border-red-500 flex flex-col items-center justify-center gap-2 text-white">
              <Upload class="w-8 h-8 text-red-400 animate-bounce" />
              <span class="text-xs font-black uppercase tracking-wider">Drop Skin or Cape PNG here</span>
            </div>
          {/if}
        </div>

        <!-- Camera Angle Preset Pills -->
        <div class="mt-4 w-full flex items-center justify-between gap-1 bg-white/4 p-1 rounded-xl border border-white/6">
          <button
            type="button"
            onclick={() => setCameraAngle(0)}
            class="flex-1 py-1 rounded-lg text-[11px] font-bold text-neutral-300 hover:text-white hover:bg-white/10 transition-all cursor-pointer"
            title="Front View"
          >
            Front
          </button>
          <button
            type="button"
            onclick={() => setCameraAngle(Math.PI)}
            class="flex-1 py-1 rounded-lg text-[11px] font-bold text-neutral-300 hover:text-white hover:bg-white/10 transition-all cursor-pointer"
            title="Back View"
          >
            Back
          </button>
          <button
            type="button"
            onclick={() => setCameraAngle(-Math.PI / 2)}
            class="flex-1 py-1 rounded-lg text-[11px] font-bold text-neutral-300 hover:text-white hover:bg-white/10 transition-all cursor-pointer"
            title="Left Side"
          >
            Left
          </button>
          <button
            type="button"
            onclick={() => setCameraAngle(Math.PI / 2)}
            class="flex-1 py-1 rounded-lg text-[11px] font-bold text-neutral-300 hover:text-white hover:bg-white/10 transition-all cursor-pointer"
            title="Right Side"
          >
            Right
          </button>
          <button
            type="button"
            onclick={toggleAutoRotate}
            class="px-2 py-1 rounded-lg text-[11px] font-bold transition-all cursor-pointer {isAutoRotating
              ? 'bg-cyan-600 text-white shadow-md'
              : 'text-neutral-400 hover:text-white'}"
            title="Auto Rotate 360"
          >
            Spin
          </button>
          <button
            type="button"
            onclick={initViewer}
            class="px-2 py-1 rounded-lg text-[11px] font-bold text-neutral-400 hover:text-white transition-all cursor-pointer"
            title="Reset Camera Angle"
          >
            <RotateCcw class="w-3 h-3" />
          </button>
        </div>

        <!-- Pose Animation & Arm Thickness -->
        <div class="mt-3 w-full space-y-2.5">
          <!-- Animation Modes -->
          <div class="flex items-center justify-between gap-1 bg-white/4 p-1 rounded-xl border border-white/6">
            <button
              type="button"
              onclick={() => applyAnimation("walk")}
              class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {animationMode === 'walk'
                ? 'bg-red-600 text-white shadow-md'
                : 'text-neutral-400 hover:text-white'}"
            >
              Walk
            </button>
            <button
              type="button"
              onclick={() => applyAnimation("run")}
              class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {animationMode === 'run'
                ? 'bg-red-600 text-white shadow-md'
                : 'text-neutral-400 hover:text-white'}"
            >
              Run
            </button>
            <button
              type="button"
              onclick={() => applyAnimation("fly")}
              class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {animationMode === 'fly'
                ? 'bg-red-600 text-white shadow-md'
                : 'text-neutral-400 hover:text-white'}"
            >
              Fly
            </button>
            <button
              type="button"
              onclick={() => applyAnimation("idle")}
              class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {animationMode === 'idle'
                ? 'bg-red-600 text-white shadow-md'
                : 'text-neutral-400 hover:text-white'}"
            >
              Idle
            </button>
            {#if animationMode !== "idle"}
              <button
                type="button"
                onclick={togglePause}
                class="px-2.5 py-1.5 rounded-lg text-xs font-bold bg-white/6 hover:bg-white/12 text-white transition-all cursor-pointer"
                title={isPaused ? "Resume Animation" : "Pause Animation"}
              >
                {#if isPaused}
                  <Play class="w-3.5 h-3.5 text-emerald-400" />
                {:else}
                  <Pause class="w-3.5 h-3.5 text-amber-400" />
                {/if}
              </button>
            {/if}
          </div>

          <!-- Arm Model Type Selection -->
          <div class="pt-1 w-full">
            <div class="flex items-center gap-1 bg-white/4 p-1 rounded-xl border border-white/6 w-full">
              <button
                type="button"
                onclick={() => setModelType("classic")}
                class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {modelType === 'classic'
                  ? 'bg-white/15 text-white border border-white/20 shadow-sm'
                  : 'text-neutral-400 hover:text-white'}"
              >
                Classic 4px
              </button>
              <button
                type="button"
                onclick={() => setModelType("slim")}
                class="flex-1 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer {modelType === 'slim'
                  ? 'bg-white/15 text-white border border-white/20 shadow-sm'
                  : 'text-neutral-400 hover:text-white'}"
              >
                Slim 3px
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Right: Content Panes (7 Cols) -->
    <div class="lg:col-span-7 space-y-5">
      <!-- GUID & In-Game Universal Sync Status Banner -->
      <div class="p-5 rounded-2xl bg-gradient-to-r from-red-950/40 via-[#0e1017] to-neutral-900/60 border border-red-500/30 shadow-lg space-y-3">
        <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div class="flex items-center gap-2.5">
            <div class="w-8 h-8 rounded-xl bg-red-500/15 border border-red-500/30 flex items-center justify-center text-red-400 shrink-0">
              <Shield class="w-4 h-4" />
            </div>
            <div>
              <h4 class="text-xs font-black text-white uppercase tracking-wider flex items-center gap-2">
                <span>Universal In-Game Skin Sync</span>
                <span class="px-2 py-0.5 rounded-full text-[9px] font-bold bg-emerald-500/15 text-emerald-400 border border-emerald-500/30 uppercase tracking-widest">
                  Active
                </span>
              </h4>
              <p class="text-[11px] text-neutral-300 mt-0.5 leading-relaxed">
                Your custom skin is mapped to your account GUID <span class="font-mono text-red-300 font-bold">({launcher.activeAccount?.id ? launcher.activeAccount.id.substring(0, 13) + '...' : 'Local'})</span> and automatically injected for all Minecraft versions (1.5 - 1.21+) across Vanilla, Fabric, Forge, NeoForge, and Quilt.
              </p>
            </div>
          </div>

          <button
            type="button"
            onclick={() => launcher.forceSyncUniversalSkin()}
            class="px-3.5 py-2 rounded-xl bg-white/8 hover:bg-white/16 border border-white/10 text-neutral-200 hover:text-white font-bold text-xs flex items-center gap-1.5 transition-all cursor-pointer active:scale-95 shrink-0 self-end sm:self-center shadow-sm"
            title="Force synchronization into options.txt and in-game mod caches"
          >
            <RefreshCw class="w-3.5 h-3.5 text-red-400" />
            <span>Force Sync</span>
          </button>
        </div>

        <div class="flex flex-wrap items-center gap-2 pt-1 border-t border-white/6 text-[10px] text-neutral-400">
          <span class="flex items-center gap-1 text-neutral-300">
            <Check class="w-3 h-3 text-red-400" /> Options.txt Auto-Linked
          </span>
          <span class="text-white/20">•</span>
          <span class="flex items-center gap-1 text-neutral-300">
            <Check class="w-3 h-3 text-red-400" /> Offline Mod Caches Synced
          </span>
          <span class="text-white/20">•</span>
          <span class="flex items-center gap-1 text-neutral-300">
            <Check class="w-3 h-3 text-red-400" /> Backward Compatible
          </span>
        </div>
      </div>

      <!-- SMP & Multiplayer Skin Visibility Card -->
      <div class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 space-y-4 shadow-xl">
        <div class="flex items-center justify-between pb-3 border-b border-white/8">
          <div class="flex items-center gap-2 text-white font-bold text-sm uppercase tracking-wider">
            <Globe class="w-4 h-4 text-cyan-400" />
            <span>SMP & Multiplayer Skin Visibility</span>
          </div>
          <span class="px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 uppercase tracking-widest">
            Multiplayer Mode
          </span>
        </div>

        <p class="text-xs text-neutral-300 leading-relaxed">
          Ensure your custom skin renders seamlessly on offline, cracked, and modded SMP multiplayer servers.
        </p>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-1">
          <!-- Option 1: 1-Click SMP Skin Mod Integration -->
          <div class="p-4 rounded-xl bg-white/2 border border-white/8 flex flex-col justify-between space-y-3">
            <div>
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-white flex items-center gap-1.5">
                  <Zap class="w-3.5 h-3.5 text-amber-400" />
                  <span>Automatic SMP Skin Mod</span>
                </span>
                {#if launcher.isSmpSkinModInstalled()}
                  <span class="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 text-[10px] font-bold">
                    ✓ Installed
                  </span>
                {/if}
              </div>
              <p class="text-[11px] text-neutral-400 mt-1 leading-relaxed">
                Loads your local skin automatically across all offline SMP and LAN servers without needing server commands.
              </p>
            </div>

            <div>
              {#if launcher.isSmpSkinModInstalled()}
                <div class="text-[11px] text-emerald-400 font-semibold flex items-center gap-1.5 bg-emerald-950/30 border border-emerald-500/20 rounded-lg px-2.5 py-1.5">
                  <CheckCircle2 class="w-3.5 h-3.5 shrink-0" />
                  <span>Active & Synced with Skin Studio</span>
                </div>
              {:else}
                <button
                  type="button"
                  onclick={() => launcher.installSmpSkinMod()}
                  disabled={launcher.isInstallingSmpSkinMod}
                  class="w-full py-2 px-3 rounded-xl bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 disabled:opacity-50 text-white font-bold text-xs uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md shadow-cyan-600/20 active:scale-95"
                >
                  {#if launcher.isInstallingSmpSkinMod}
                    <Loader2 class="w-3.5 h-3.5 animate-spin" />
                    <span>Installing Mod...</span>
                  {:else}
                    <Download class="w-3.5 h-3.5" />
                    <span>Install SMP Skin Mod (1-Click)</span>
                  {/if}
                </button>
              {/if}
            </div>
          </div>

          <!-- Option 2: In-Game SMP Command Helper -->
          <div class="p-4 rounded-xl bg-white/2 border border-white/8 flex flex-col justify-between space-y-3">
            <div>
              <span class="text-xs font-bold text-white flex items-center gap-1.5">
                <Shirt class="w-3.5 h-3.5 text-purple-400" />
                <span>SkinsRestorer Server Command</span>
              </span>
              <p class="text-[11px] text-neutral-400 mt-1 leading-relaxed">
                On SMP servers running the SkinsRestorer plugin, run this in server chat so every player sees your skin:
              </p>
            </div>

            <div class="flex items-center gap-2 bg-black/40 px-3 py-2 rounded-xl border border-white/6 justify-between">
              <span class="text-xs font-mono text-purple-300 font-bold truncate">
                /skin {launcher.activeAccount?.username || "Player"}
              </span>
              <button
                type="button"
                onclick={() => launcher.copySmpSkinCommand()}
                class="px-2.5 py-1 rounded-lg bg-white/10 hover:bg-white/20 text-neutral-200 hover:text-white text-[11px] font-semibold flex items-center gap-1 transition-colors cursor-pointer shrink-0"
                title="Copy /skin command to clipboard"
              >
                <Copy class="w-3 h-3" />
                <span>Copy</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Active Custom Skin & Cape Uploader Card -->
      <div class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 space-y-4">
        <div class="flex items-center justify-between pb-3 border-b border-white/8">
          <div class="flex items-center gap-2 text-white font-bold text-sm uppercase tracking-wider">
            <Upload class="w-4 h-4 text-red-500" />
            <span>Apply Custom Textures</span>
          </div>
          <button
            type="button"
            onclick={downloadCurrentSkin}
            class="text-xs text-neutral-400 hover:text-white flex items-center gap-1.5 transition-colors cursor-pointer"
            title="Download your active skin texture PNG"
          >
            <Download class="w-3.5 h-3.5" />
            <span>Save PNG</span>
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Skin Uploader -->
          <button
            type="button"
            onclick={() => launcher.pickAndApplySkin()}
            class="p-5 rounded-2xl bg-white/2 hover:bg-red-500/10 border border-white/8 hover:border-red-500/30 text-left transition-all cursor-pointer group flex flex-col justify-between"
          >
            <div>
              <div class="w-10 h-10 rounded-xl bg-red-500/10 border border-red-500/20 flex items-center justify-center text-red-400 group-hover:scale-105 transition-transform mb-3">
                <Shirt class="w-5 h-5" />
              </div>
              <div class="text-sm font-bold text-white group-hover:text-red-400 transition-colors">
                Choose Skin File
              </div>
              <p class="text-xs text-neutral-400 mt-1 leading-relaxed">
                Select any standard 64x64 PNG Minecraft skin. Drag & drop directly onto the 3D model also supported.
              </p>
            </div>

            <div class="mt-4 flex items-center gap-1.5 text-xs text-red-400 font-bold uppercase tracking-wider">
              <span>Browse File</span>
              <ArrowUpRight class="w-3.5 h-3.5" />
            </div>
          </button>

          <!-- Cape Uploader -->
          <button
            type="button"
            onclick={() => launcher.pickAndApplyCape()}
            class="p-5 rounded-2xl bg-white/2 hover:bg-purple-500/10 border border-white/8 hover:border-purple-500/30 text-left transition-all cursor-pointer group flex flex-col justify-between"
          >
            <div>
              <div class="w-10 h-10 rounded-xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center text-purple-400 group-hover:scale-105 transition-transform mb-3">
                <Sparkle class="w-5 h-5" />
              </div>
              <div class="text-sm font-bold text-white group-hover:text-purple-400 transition-colors">
                Choose Custom Cape File
              </div>
              <p class="text-xs text-neutral-400 mt-1 leading-relaxed">
                Select standard 64x32 PNG cape cosmetics. Injected natively into game launch resource packs.
              </p>
            </div>

            <div class="mt-4 flex items-center gap-1.5 text-xs text-purple-400 font-bold uppercase tracking-wider">
              <span>Browse File</span>
              <ArrowUpRight class="w-3.5 h-3.5" />
            </div>
          </button>
        </div>

        <!-- Quick Reset Toolbar -->
        <div class="flex items-center gap-3 pt-2">
          <button
            type="button"
            onclick={() => launcher.resetPlayerSkin()}
            class="flex-1 py-2 px-3 rounded-xl bg-white/4 hover:bg-white/8 border border-white/8 text-neutral-300 hover:text-white text-xs font-semibold flex items-center justify-center gap-2 transition-colors cursor-pointer"
          >
            <RotateCcw class="w-3.5 h-3.5" />
            <span>Reset Skin to Steve</span>
          </button>

          {#if launcher.activeCapeDataUrl}
            <button
              type="button"
              onclick={() => launcher.resetPlayerCape()}
              class="flex-1 py-2 px-3 rounded-xl bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/20 text-rose-300 text-xs font-semibold flex items-center justify-center gap-2 transition-colors cursor-pointer"
            >
              <Trash2 class="w-3.5 h-3.5" />
              <span>Remove Active Cape</span>
            </button>
          {/if}
        </div>
      </div>

      <!-- Technical Diagnostics Card -->
      <div class="p-5 rounded-2xl bg-[#0e1017]/90 border border-white/8 space-y-3">
        <div class="text-xs text-neutral-400 font-bold uppercase tracking-wider">
          Active Texture Specifications
        </div>
        <div class="grid grid-cols-3 gap-3">
          <div class="p-3 rounded-xl bg-white/2 border border-white/6">
            <span class="text-[10px] text-neutral-400 block uppercase font-bold">Skin Format</span>
            <span class="text-xs font-mono font-bold text-emerald-400">64x64 RGBA</span>
          </div>
          <div class="p-3 rounded-xl bg-white/2 border border-white/6">
            <span class="text-[10px] text-neutral-400 block uppercase font-bold">Model Rig</span>
            <span class="text-xs font-mono font-bold text-white capitalize">{modelType} Arm</span>
          </div>
          <div class="p-3 rounded-xl bg-white/2 border border-white/6">
            <span class="text-[10px] text-neutral-400 block uppercase font-bold">In-Game Sync</span>
            <span class="text-xs font-mono font-bold text-emerald-400">All Versions</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
