<script lang="ts">
  import { launcher } from "../../state/launcher.svelte";
  import {
    Gamepad2,
    FolderKanban,
    Wrench,
    User,
    Settings,
    Terminal,
    Coffee,
    Cpu,
    Info,
  } from "lucide-svelte";
</script>

<aside
  class="w-14 sm:w-16 md:w-56 lg:w-64 border-r border-white/8 bg-[#0a0c12]/95 backdrop-blur-2xl flex flex-col justify-between p-2 md:p-3.5 lg:p-4 shrink-0 shadow-2xl z-20 overflow-y-auto overflow-x-hidden"
>
  <div class="space-y-4 md:space-y-6">
    <!-- Brand Crest Header -->
    <div class="flex items-center gap-3 px-1 md:px-2 pt-1 md:pt-2 justify-center md:justify-start">
      <div
        class="relative w-9 h-9 md:w-10 md:h-10 rounded-2xl overflow-hidden bg-black/40 border border-white/10 flex items-center justify-center shadow-lg shadow-red-500/15 group shrink-0"
      >
        <img
          src="/64x64.png"
          alt="Digital Vigital Logo"
          class="w-full h-full object-contain p-1 group-hover:scale-105 transition-transform"
        />
        <span
          class="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 md:w-3 md:h-3 rounded-full bg-emerald-500 border-2 border-[#0a0c12]"
          title="Online / Ready"
        ></span>
      </div>
      <div class="hidden md:block">
        <h1 class="font-bold text-sm tracking-tight uppercase text-white truncate">
          Digital Vigital
        </h1>
        <div
          class="flex items-center gap-1.5 text-xs text-neutral-400 font-medium"
        >
          <span>v{launcher.launcherInfo.version}</span>
        </div>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <nav class="space-y-1.5 font-sans">
      <button
        onclick={() => (launcher.activeTab = "play")}
        title="Launch & Play"
        class="w-full flex items-center justify-center md:justify-start gap-3.5 px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'play'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <Gamepad2
          class="w-4 h-4 shrink-0 {launcher.activeTab === 'play'
            ? 'text-red-400'
            : 'text-neutral-400'}"
        />
        <span class="hidden md:inline">Launch & Play</span>
      </button>

      <button
        onclick={() => (launcher.activeTab = "instances")}
        title="Profiles"
        class="w-full relative flex items-center justify-center md:justify-between px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'instances'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <div class="flex items-center gap-3.5">
          <FolderKanban
            class="w-4 h-4 shrink-0 {launcher.activeTab === 'instances'
              ? 'text-red-400'
              : 'text-neutral-400'}"
          />
          <span class="hidden md:inline">Profiles</span>
        </div>
        <span
          class="hidden md:inline-block px-2 py-0.5 rounded-md bg-white/8 font-mono text-xs text-neutral-300 font-medium"
        >
          {launcher.instances.length}
        </span>
      </button>

      <button
        onclick={() => (launcher.activeTab = "mods")}
        title="Mod Studio"
        class="w-full relative flex items-center justify-center md:justify-between px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'mods'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <div class="flex items-center gap-3.5">
          <Wrench
            class="w-4 h-4 shrink-0 {launcher.activeTab === 'mods'
              ? 'text-red-400'
              : 'text-neutral-400'}"
          />
          <span class="hidden md:inline">Mod</span>
        </div>
        {#if launcher.installedMods.length > 0}
          <span
            class="hidden md:inline-block px-2 py-0.5 rounded-md bg-red-500/20 text-red-400 font-mono text-xs font-bold"
          >
            {launcher.installedMods.length}
          </span>
          <span class="md:hidden absolute top-2 right-2 w-2 h-2 rounded-full bg-red-500"></span>
        {/if}
      </button>

      <button
        onclick={() => (launcher.activeTab = "skins")}
        title="Skins"
        class="w-full flex items-center justify-center md:justify-start gap-3.5 px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'skins'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <User
          class="w-4 h-4 shrink-0 {launcher.activeTab === 'skins'
            ? 'text-red-400'
            : 'text-neutral-400'}"
        />
        <span class="hidden md:inline">Skins</span>
      </button>

      <button
        onclick={() => (launcher.activeTab = "settings")}
        title="Settings"
        class="w-full flex items-center justify-center md:justify-start gap-3.5 px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'settings'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <Settings
          class="w-4 h-4 shrink-0 {launcher.activeTab === 'settings'
            ? 'text-red-400'
            : 'text-neutral-400'}"
        />
        <span class="hidden md:inline">Settings</span>
      </button>

      <button
        onclick={() => (launcher.activeTab = "console")}
        title="Console"
        class="w-full relative flex items-center justify-center md:justify-between px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'console'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <div class="flex items-center gap-3.5">
          <Terminal
            class="w-4 h-4 shrink-0 {launcher.activeTab === 'console'
              ? 'text-red-400'
              : 'text-neutral-400'}"
          />
          <span class="hidden md:inline">Console</span>
        </div>
        {#if launcher.gameStatus === "running"}
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-ping shrink-0"
          ></span>
        {/if}
      </button>

      <button
        onclick={() => (launcher.activeTab = "about")}
        title="About"
        class="w-full flex items-center justify-center md:justify-start gap-3.5 px-2 md:px-3.5 py-2.5 md:py-3 rounded-xl text-xs font-bold uppercase tracking-wider transition-all cursor-pointer {launcher.activeTab ===
        'about'
          ? 'bg-red-500/15 text-red-400 border border-red-500/30 shadow-lg shadow-red-500/10'
          : 'text-neutral-300 hover:text-white hover:bg-white/5'}"
      >
        <Info
          class="w-4 h-4 shrink-0 {launcher.activeTab === 'about'
            ? 'text-red-400'
            : 'text-neutral-400'}"
        />
        <span class="hidden md:inline">About</span>
      </button>
    </nav>
  </div>

  <!-- Quick Status Hardware Footer (Desktop) -->
  <div
    class="hidden md:block p-3.5 rounded-2xl bg-white/4 border border-white/8 space-y-2.5 text-xs text-neutral-300"
  >
    {#if !launcher.isMobile}
      <div class="flex items-center justify-between">
        <span class="flex items-center gap-2 text-neutral-400"
          ><Coffee class="w-4 h-4 text-amber-400" /> Java</span
        >
        <span class="text-neutral-100 font-bold truncate max-w-[100px]">
          {launcher.settings.custom_java_path
            ? (launcher.detectedJavas.find((j) => j.path === launcher.settings.custom_java_path)?.display_name || "Custom Java")
            : "Auto-Detect"}
        </span>
      </div>
      <div class="flex items-center justify-between">
        <span class="flex items-center gap-2 text-neutral-400"
          ><Cpu class="w-4 h-4 text-sky-400" /> GPU</span
        >
        <span class="text-neutral-100 font-bold truncate max-w-[100px]">
          {launcher.detectedGpus
            .find((g) => g.is_discrete)
            ?.name.replace(/NVIDIA|GeForce|AMD|Radeon/gi, "")
            .trim() || "Dedicated"}
        </span>
      </div>
    {:else}
      <div class="flex items-center justify-between">
        <span class="text-neutral-400">Platform</span>
        <span class="text-emerald-400 font-bold text-[11px] uppercase tracking-wider">Android ARM64</span>
      </div>
    {/if}
    <div
      class="flex items-center justify-between pt-2 border-t border-white/8 text-xs"
    >
      <span class="text-neutral-400">Status</span>
      {#if launcher.gameStatus === "running"}
        <span class="text-emerald-400 font-bold flex items-center gap-1.5">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"
          ></span>
          Running
        </span>
      {:else if launcher.gameStatus === "launching" || launcher.gameStatus === "downloading"}
        <span class="text-amber-400 font-bold flex items-center gap-1.5">
          <span class="w-2 h-2 rounded-full bg-amber-400 animate-spin"></span>
          Starting
        </span>
      {:else}
        <span class="text-neutral-400 flex items-center gap-1.5">
          <span class="w-2 h-2 rounded-full bg-neutral-600"></span>
          Standby
        </span>
      {/if}
    </div>
  </div>

  <!-- Mobile Compact Status Indicator Dot -->
  <div class="md:hidden flex justify-center py-2" title="Status: {launcher.gameStatus}">
    {#if launcher.gameStatus === "running"}
      <span class="w-3 h-3 rounded-full bg-emerald-400 animate-pulse" title="Running"></span>
    {:else if launcher.gameStatus === "launching" || launcher.gameStatus === "downloading"}
      <span class="w-3 h-3 rounded-full bg-amber-400 animate-spin" title="Starting"></span>
    {:else}
      <span class="w-2.5 h-2.5 rounded-full bg-neutral-600" title="Standby"></span>
    {/if}
  </div>
</aside>
