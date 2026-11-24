<script lang="ts">
  import { launcher } from "../../state/launcher.svelte";
  import {
    Play,
    Square,
    Sparkles,
    Zap,
    FolderOpen,
    Wrench,
    CheckCircle2,
    HardDrive,
    Sliders,
    FolderTree,
    ArrowUpRight,
    AlertTriangle,
    Send,
    Loader2,
    Check,
    Terminal,
    Download,
    X,
    FileText,
    Info,
    User,
    Trash2,
    RefreshCw,
    Image,
    Camera,
    Map,
    Archive,
    Eye,
  } from "lucide-svelte";

  let showWorldsModal = $state(false);
  let showScreenshotsModal = $state(false);
  let selectedPreviewImage = $state<string | null>(null);

  function getLoaderColor(loader: string = "") {
    const l = loader.toLowerCase();
    if (l.includes("fabric"))
      return "text-purple-300 border-purple-500/30 bg-purple-500/15";
    if (l.includes("neoforge"))
      return "text-orange-300 border-orange-500/30 bg-orange-500/15";
    if (l.includes("forge"))
      return "text-amber-300 border-amber-500/30 bg-amber-500/15";
    return "text-neutral-300 border-neutral-500/30 bg-neutral-500/15";
  }

  function getRequiredJava(mcVersion: string = ""): number {
    const parts = mcVersion.split(".");
    if (parts.length >= 2) {
      const minor = parseInt(parts[1], 10) || 21;
      const patch = parseInt(parts[2], 10) || 0;
      if (minor >= 21 || (minor === 20 && patch >= 5)) return 21;
      if (minor >= 17) return 17;
      return 8;
    }
    return 21;
  }

  const reqJava = $derived(
    launcher.activeInstance ? getRequiredJava(launcher.activeInstance.mc_version) : 21
  );
  const hasCompatibleJava = $derived(
    launcher.detectedJavas.some((j) => j.major_version >= reqJava)
  );
</script>

<div class="h-full overflow-y-auto p-4 sm:p-6 lg:p-8 space-y-5 sm:space-y-6">
  <!-- Software Update Notification Banner -->
  {#if launcher.updateInfo?.has_update && !launcher.dismissedUpdateBanner}
    <div
      class="p-4 rounded-2xl bg-gradient-to-r from-emerald-950/70 via-[#0d1f17]/60 to-[#0a1611]/80 border border-emerald-500/35 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 shadow-xl"
    >
      <div class="flex items-center gap-3.5">
        <div
          class="w-10 h-10 rounded-xl bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400 shrink-0 shadow-md shadow-emerald-500/10"
        >
          <Sparkles class="w-5 h-5" />
        </div>
        <div>
          <div class="text-sm font-bold text-white flex items-center gap-2">
            <span>Digital Vigital {launcher.updateInfo.latest_version} is Available!</span>
            <span
              class="px-2 py-0.5 rounded-full bg-emerald-500/25 text-emerald-300 text-[10px] font-black uppercase tracking-wider"
            >
              Update Ready
            </span>
          </div>
          <p class="text-xs text-neutral-300 mt-0.5">
            You are running v{launcher.updateInfo.current_version}. Upgrade now for the latest features, fixes, and game optimizations.
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 shrink-0 w-full md:w-auto">
        <button
          onclick={() => launcher.openReleaseUrl()}
          class="flex-1 md:flex-none px-4 py-2 rounded-xl bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-bold text-xs uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer shadow-lg shadow-emerald-600/30"
        >
          <Download class="w-3.5 h-3.5" />
          <span>Download Update</span>
        </button>

        <button
          onclick={() => (launcher.showUpdateModal = true)}
          class="px-3.5 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-neutral-200 hover:text-white font-bold text-xs flex items-center gap-1.5 transition-all cursor-pointer border border-white/10"
        >
          <FileText class="w-3.5 h-3.5" />
          <span>Changelog</span>
        </button>

        <button
          onclick={() => (launcher.dismissedUpdateBanner = true)}
          class="p-2 rounded-xl bg-white/5 hover:bg-white/10 text-neutral-400 hover:text-white transition-all cursor-pointer"
          title="Dismiss notification for this session"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </div>
  {/if}

  <!-- Main Launch Hero Card -->
  <div
    class="relative rounded-3xl overflow-hidden border border-white/10 bg-gradient-to-br from-[#12141c] via-[#0c0d14] to-[#18090d] shadow-2xl p-8 flex flex-col justify-between min-h-[300px]"
  >
    <!-- Ambient Red Specular Flare -->
    <div
      class="absolute -top-24 -right-24 w-96 h-96 bg-red-600/15 rounded-full blur-3xl pointer-events-none"
    ></div>
    <div
      class="absolute bottom-0 right-1/4 w-80 h-40 bg-purple-600/10 rounded-full blur-3xl pointer-events-none"
    ></div>

    <!-- Top Info Row in Hero -->
    <div class="relative z-10 flex items-start justify-between">
      <div>
        <div
          class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-red-500/10 border border-red-500/20 text-red-400 text-xs font-bold tracking-wide uppercase mb-3"
        >
          <Zap class="w-3.5 h-3.5 fill-red-400" />
          <span>Ultra-Low Latency Engine</span>
        </div>
        <h2 class="text-3xl font-extrabold text-white tracking-tight">
          {launcher.activeInstance
            ? launcher.activeInstance.name
            : "Select or Create Profile"}
        </h2>
        <div
          class="flex items-center gap-3 mt-2.5 text-xs text-neutral-300 font-medium"
        >
          {#if launcher.activeInstance}
            <span
              class="px-2.5 py-0.5 rounded border {getLoaderColor(
                launcher.activeInstance.mod_loader,
              )} font-bold uppercase tracking-wide"
            >
              {launcher.activeInstance.mod_loader}
            </span>
            <span class="text-neutral-500">•</span>
            <span class="text-white font-semibold"
              >Minecraft {launcher.activeInstance.mc_version}</span
            >
            <span class="text-neutral-500">•</span>
            <span class="text-neutral-300"
              >{launcher.installedMods.length} mods installed</span
            >
          {:else}
            <span class="text-neutral-400"
              >Pick an active profile to begin playing</span
            >
          {/if}
        </div>
      </div>

      <!-- Quick Action Pill for Mods & Folder -->
      {#if launcher.activeInstance}
        <div class="flex items-center gap-2">
          <button
            onclick={() => launcher.openModsFolder()}
            title="Open instance folder"
            class="px-3.5 py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 text-neutral-200 hover:text-white text-xs font-semibold flex items-center gap-2 transition-all cursor-pointer"
          >
            <FolderOpen class="w-4 h-4" />
            <span>Folder</span>
          </button>
          <button
            onclick={() => (launcher.activeTab = "mods")}
            title="Open Mod Studio"
            class="px-3.5 py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 text-neutral-200 hover:text-white text-xs font-semibold flex items-center gap-2 transition-all cursor-pointer"
          >
            <Wrench class="w-4 h-4" />
            <span>Mods</span>
          </button>
        </div>
      {/if}
    </div>

    <!-- Fatal Crash Alert Card (Only shown for real fatal errors) -->
    {#if !launcher.isCrashDismissed && launcher.hasCrashDetected}
      <div
        class="relative z-10 p-4 sm:p-5 rounded-2xl bg-rose-950/85 border border-rose-500/50 shadow-2xl backdrop-blur-md animate-in fade-in duration-200"
      >
        <div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4">
          <div class="flex items-start gap-3.5 flex-1 min-w-0">
            <div
              class="w-10 h-10 rounded-xl bg-rose-500/20 border border-rose-500/40 flex items-center justify-center text-rose-400 shrink-0 mt-0.5"
            >
              <AlertTriangle class="w-5 h-5" />
            </div>
            <div class="flex-1 min-w-0 space-y-1">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-sm font-bold text-white tracking-tight">{launcher.crashDetails?.headline || "Minecraft Crash Detected"}</span>
                <span
                  class="px-2 py-0.5 rounded bg-rose-500/25 border border-rose-500/40 text-rose-300 text-[10px] font-black uppercase tracking-wider"
                >
                  {launcher.crashDetails?.category || "Crash Alert"}
                </span>
              </div>
              <p class="text-xs text-neutral-200 leading-relaxed pr-2">
                {launcher.crashDetails?.explanation || "The game process terminated with an error. Review the Live Console or report diagnostics."}
              </p>
              {#if launcher.crashDetails?.recommended_action}
                <div class="mt-2 text-xs text-amber-200 bg-amber-950/50 border border-amber-500/35 rounded-lg px-3 py-1.5 flex items-center gap-2">
                  <span class="font-bold uppercase tracking-wider text-[10px] text-amber-400 shrink-0">Fix:</span>
                  <span class="line-clamp-2">{launcher.crashDetails.recommended_action}</span>
                </div>
              {/if}
            </div>
          </div>

          <div class="flex items-center gap-2.5 w-full sm:w-auto shrink-0 flex-nowrap justify-start sm:justify-end pt-1 lg:pt-0">
            {#if launcher.crashDetails?.category?.includes("Java")}
              <button
                onclick={() => (launcher.activeTab = "settings")}
                class="px-3.5 py-2 rounded-xl bg-amber-600 hover:bg-amber-500 text-neutral-900 font-bold text-xs flex items-center gap-1.5 transition-all cursor-pointer shadow-md active:scale-95 shrink-0"
              >
                <span>Java Settings</span>
              </button>
            {:else if launcher.crashDetails?.category?.includes("Mod")}
              <button
                onclick={() => (launcher.activeTab = "mods")}
                class="px-3.5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-bold text-xs flex items-center gap-1.5 transition-all cursor-pointer shadow-md active:scale-95 shrink-0"
              >
                <span>Mod Studio</span>
              </button>
            {/if}

            <button
              onclick={() => launcher.reportCrash()}
              disabled={launcher.isReportingCrash}
              class="px-4 py-2 rounded-xl bg-gradient-to-r from-rose-600 to-red-600 hover:from-rose-500 hover:to-red-500 disabled:opacity-50 text-white font-bold text-xs uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer shadow-lg shadow-rose-600/30 active:scale-95 shrink-0"
            >
              {#if launcher.isReportingCrash}
                <Loader2 class="w-3.5 h-3.5 animate-spin" />
                <span>Reporting...</span>
              {:else if launcher.crashReportSuccess}
                <Check class="w-3.5 h-3.5 text-emerald-300" />
                <span>Reported!</span>
              {:else}
                <Send class="w-3.5 h-3.5" />
                <span>Report Crash</span>
              {/if}
            </button>

            <button
              onclick={() => (launcher.activeTab = "console")}
              class="px-3.5 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-neutral-200 hover:text-white font-bold text-xs flex items-center gap-1.5 transition-all cursor-pointer border border-white/10 active:scale-95 shrink-0"
            >
              <Terminal class="w-3.5 h-3.5" />
              <span>Console</span>
            </button>

            <div class="hidden sm:block w-px h-6 bg-white/10 mx-0.5 shrink-0"></div>

            <button
              onclick={() => launcher.dismissCrash()}
              class="p-2 rounded-xl bg-white/8 hover:bg-white/16 border border-white/10 text-neutral-400 hover:text-white flex items-center justify-center transition-all cursor-pointer active:scale-95 shadow-sm shrink-0"
              title="Dismiss alert"
              aria-label="Dismiss alert"
            >
              <X class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    {/if}

    <!-- Non-Fatal Session Hint Banner (For small warnings or abnormal exits) -->
    {#if !launcher.isCrashDismissed && launcher.hintDetails && launcher.hintDetails.exit_code !== 0}
      {@const hint = launcher.hintDetails}
      <div
        class="relative z-10 p-4 sm:p-5 rounded-2xl bg-gradient-to-r from-neutral-900/90 via-[#131622]/90 to-[#0e1017]/90 border border-white/12 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 shadow-xl backdrop-blur-md animate-in fade-in duration-200"
      >
        <div class="flex items-center gap-3 min-w-0 pr-2">
          <div
            class="w-9 h-9 rounded-xl bg-cyan-500/15 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shrink-0"
          >
            <Info class="w-4 h-4" />
          </div>
          <div class="min-w-0">
            <div class="text-xs font-bold text-white flex items-center gap-2">
              <span>{hint?.headline || "Minecraft Session Ended"}</span>
              <span class="px-2 py-0.2 rounded bg-white/8 text-neutral-300 text-[10px] font-mono">
                Exit Code {hint?.exit_code ?? 0}
              </span>
            </div>
            <p class="text-[11px] text-neutral-300 mt-0.5 leading-snug">
              {hint?.explanation || "The game process closed. If you closed the window intentionally, no action is needed."}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0 flex-nowrap self-end sm:self-center">
          <button
            onclick={() => (launcher.activeTab = "console")}
            class="px-3 py-1.5 rounded-lg bg-white/6 hover:bg-white/12 text-neutral-300 hover:text-white text-xs font-semibold flex items-center gap-1.5 transition-colors cursor-pointer border border-white/8 active:scale-95 shrink-0"
          >
            <Terminal class="w-3.5 h-3.5 text-neutral-400" />
            <span>View Logs</span>
          </button>

          <div class="w-px h-5 bg-white/10 mx-0.5 shrink-0"></div>

          <button
            onclick={() => { launcher.dismissCrash(); launcher.dismissHint(); }}
            class="p-2 rounded-xl bg-white/8 hover:bg-white/16 border border-white/10 text-neutral-400 hover:text-white flex items-center justify-center transition-all cursor-pointer active:scale-95 shadow-sm shrink-0"
            title="Dismiss notice"
            aria-label="Dismiss notice"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </div>
    {/if}

    <!-- 1-Click Java Auto-Install Readiness Card -->
    {#if launcher.activeInstance && !hasCompatibleJava && launcher.gameStatus !== "running"}
      <div
        class="relative z-10 p-3.5 rounded-2xl bg-gradient-to-r from-amber-950/60 via-[#1e1508]/60 to-[#120d05]/70 border border-amber-500/35 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 shadow-xl backdrop-blur-sm"
      >
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="w-9 h-9 rounded-xl bg-amber-500/15 border border-amber-500/30 flex items-center justify-center text-amber-400 shrink-0"
          >
            <Sparkles class="w-4 h-4" />
          </div>
          <div class="min-w-0">
            <div class="text-xs font-bold text-white flex items-center gap-2">
              <span>Java {reqJava} (LTS) Required</span>
              <span class="px-2 py-0.2 rounded-full bg-amber-500/20 text-amber-300 text-[10px] font-mono font-bold">
                Minecraft {launcher.activeInstance.mc_version}
              </span>
            </div>
            <p class="text-[11px] text-neutral-300 mt-0.5">
              Eclipse Temurin OpenJDK {reqJava} will be automatically configured for maximum FPS.
            </p>
          </div>
        </div>

        <button
          onclick={() => launcher.installJava(reqJava)}
          disabled={launcher.isInstallingJava}
          class="shrink-0 px-4 py-2 rounded-xl bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 text-neutral-900 font-extrabold text-xs uppercase tracking-wider flex items-center gap-2 transition-all cursor-pointer shadow-lg shadow-amber-600/25 disabled:opacity-50"
        >
          {#if launcher.isInstallingJava && launcher.installingJavaVersion === reqJava}
            <Loader2 class="w-3.5 h-3.5 animate-spin text-neutral-900" />
            <span>Installing Java {reqJava}...</span>
          {:else}
            <Download class="w-3.5 h-3.5 text-neutral-900" />
            <span>1-Click Install Java {reqJava}</span>
          {/if}
        </button>
      </div>
    {/if}

    <!-- Player Username Required Banner -->
    {#if !launcher.activeAccount && launcher.gameStatus !== "running"}
      <div
        class="relative z-10 p-4 rounded-2xl bg-gradient-to-r from-red-950/70 via-[#1e0a0f]/70 to-[#120508]/80 border border-red-500/40 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 shadow-xl backdrop-blur-sm animate-in fade-in duration-300"
      >
        <div class="flex items-center gap-3.5 min-w-0">
          <div
            class="w-10 h-10 rounded-xl bg-red-500/20 border border-red-500/40 flex items-center justify-center text-red-400 shrink-0"
          >
            <User class="w-5 h-5" />
          </div>
          <div class="min-w-0">
            <div class="text-xs font-bold text-white flex items-center gap-2">
              <span>Player Username Required</span>
              <span class="px-2 py-0.5 rounded-full bg-red-500/20 text-red-300 text-[10px] font-mono font-bold">
                Online or Offline
              </span>
            </div>
            <p class="text-[11px] text-neutral-300 mt-0.5 leading-relaxed">
              Choose an offline username to jump straight into singleplayer/SMPs, or connect your Microsoft account.
            </p>
          </div>
        </div>

        <button
          onclick={() => (launcher.showAccountModal = true)}
          class="shrink-0 px-4 py-2.5 rounded-xl bg-gradient-to-r from-red-600 to-rose-600 hover:from-red-500 hover:to-rose-500 text-white font-extrabold text-xs uppercase tracking-wider flex items-center gap-2 transition-all cursor-pointer shadow-lg shadow-red-600/30 hover:scale-105 active:scale-95"
        >
          <User class="w-3.5 h-3.5" />
          <span>Choose Username</span>
        </button>
      </div>
    {/if}

    <!-- Bottom Launch Area -->
    <div
      class="relative z-10 pt-6 flex flex-col md:flex-row items-center justify-between gap-6"
    >
      <!-- Status & Download Progress -->
      <div class="w-full md:w-auto flex-1 space-y-2">
        <div class="flex items-center gap-2.5 text-sm text-neutral-200">
          {#if launcher.gameStatus === "running"}
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse"
            ></span>
          {:else if launcher.gameStatus === "launching" || launcher.gameStatus === "downloading"}
            <span class="w-2.5 h-2.5 rounded-full bg-amber-400 animate-spin"
            ></span>
          {:else if launcher.gameStatus === "crashed"}
            <span class="w-2.5 h-2.5 rounded-full bg-rose-500 animate-pulse"
            ></span>
          {:else}
            <span class="w-2.5 h-2.5 rounded-full bg-neutral-600"></span>
          {/if}
          <span class="font-medium">{launcher.launchStatusText}</span>
        </div>

        {#if launcher.gameStatus === "downloading"}
          <div
            class="w-full max-w-md h-2.5 rounded-full bg-white/8 border border-white/10 overflow-hidden relative"
          >
            <div
              class="h-full bg-gradient-to-r from-red-600 to-rose-400 rounded-full transition-all duration-200"
              style="width: {launcher.downloadProgress.percent}%"
            ></div>
          </div>
          <div
            class="flex justify-between max-w-md text-xs font-mono text-neutral-400"
          >
            <span
              >{launcher.downloadProgress.current} / {launcher.downloadProgress
                .total} items</span
            >
            <span>{launcher.downloadProgress.percent.toFixed(1)}%</span>
          </div>
        {/if}
      </div>

      <!-- Main Giant Play / Stop Button -->
      <button
        onclick={() => launcher.launchOrStop()}
        disabled={launcher.gameStatus === "launching"}
        class="w-full md:w-auto px-10 py-4 rounded-2xl font-black uppercase tracking-wider text-sm flex items-center justify-center gap-3 shadow-2xl transition-all cursor-pointer duration-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed {launcher.gameStatus ===
        'running'
          ? 'bg-rose-600 hover:bg-rose-500 text-white shadow-rose-600/30 border border-rose-400/30'
          : launcher.gameStatus === 'downloading' ||
              launcher.gameStatus === 'launching'
            ? 'bg-amber-600 text-white cursor-wait border border-amber-400/30'
            : 'bg-gradient-to-r from-red-600 to-rose-600 hover:from-red-500 hover:to-rose-500 text-white shadow-red-600/40 hover:shadow-red-500/50 hover:scale-[1.02] border border-red-400/30'}"
      >
        {#if launcher.gameStatus === "running"}
          <Square class="w-5 h-5 fill-white" />
          <span>STOP MINECRAFT</span>
        {:else if launcher.gameStatus === "launching"}
          <div
            class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"
          ></div>
          <span>LAUNCHING...</span>
        {:else if launcher.gameStatus === "downloading"}
          <div
            class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"
          ></div>
          <span>DOWNLOADING...</span>
        {:else if !launcher.activeAccount}
          <User class="w-5 h-5 text-white" />
          <span>CHOOSE USERNAME TO PLAY</span>
        {:else}
          <Play class="w-5 h-5 fill-white" />
          <span>PLAY MINECRAFT</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Quick Feature Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 sm:gap-6">
    <!-- Storage & Engine Management Card -->
    <div
      class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 hover:border-white/16 transition-all flex flex-col justify-between group"
    >
      <div>
        <div class="flex items-center justify-between mb-3">
          <div
            class="w-10 h-10 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400 group-hover:scale-110 transition-transform"
          >
            <FolderTree class="w-5 h-5" />
          </div>
          <span
            class="text-xs font-bold uppercase tracking-wider px-2.5 py-0.5 rounded bg-cyan-500/15 text-cyan-400 border border-cyan-500/20"
          >
            Multi-Drive
          </span>
        </div>
        <h3 class="font-bold text-base text-white">Storage & Engine Paths</h3>
        <p class="text-xs text-neutral-300 mt-1.5 leading-relaxed font-normal">
          Customize base game files and instance directories across your SSDs and HDDs.
        </p>
      </div>

      <button
        onclick={() => (launcher.activeTab = "settings")}
        class="mt-5 w-full py-2.5 rounded-xl bg-white/5 hover:bg-cyan-500/20 hover:border-cyan-500/30 border border-white/10 text-white text-xs font-bold uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer"
      >
        <Sliders class="w-4 h-4 text-cyan-400" />
        <span>Configure Storage</span>
      </button>
    </div>

    <!-- Active Hardware & Tuning Summary -->
    <div
      class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 hover:border-white/16 transition-all flex flex-col justify-between"
    >
      <div>
        <div class="flex items-center justify-between mb-3">
          <div
            class="w-10 h-10 rounded-xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center text-purple-400"
          >
            <HardDrive class="w-5 h-5" />
          </div>
          <span
            class="text-xs font-bold uppercase tracking-wider px-2.5 py-0.5 rounded bg-white/8 text-neutral-300 border border-white/10"
          >
            Engine Spec
          </span>
        </div>
        <h3 class="font-bold text-base text-white">Execution Parameters</h3>

        <div class="mt-3.5 space-y-2 text-xs text-neutral-300">
          <div
            class="flex justify-between items-center py-1 border-b border-white/5"
          >
            <span class="text-neutral-400">Allocated RAM</span>
            <span class="text-white font-bold font-mono"
              >{launcher.settings.default_max_memory_mb ||
                launcher.activeInstance?.max_memory_mb ||
                4096} MB ({(
                (launcher.settings.default_max_memory_mb || 4096) / 1024
              ).toFixed(1)} GB)</span
            >
          </div>
          <div
            class="flex justify-between items-center py-1 border-b border-white/5"
          >
            <span class="text-neutral-400">GC Preset</span>
            <span class="text-red-400 font-bold uppercase"
              >{launcher.settings.performance_preset}</span
            >
          </div>
          <div class="flex justify-between items-center py-1">
            <span class="text-neutral-400">Java Runtime</span>
            <span class="text-white font-bold font-mono text-xs flex items-center gap-1.5">
              {#if launcher.settings.custom_java_path}
                <span class="text-cyan-400">Custom</span>
                <span class="text-neutral-300 truncate max-w-[130px]" title={launcher.settings.custom_java_path}>
                  {launcher.detectedJavas.find((j) => j.path === launcher.settings.custom_java_path)?.display_name || "Custom Java"}
                </span>
              {:else if hasCompatibleJava}
                <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                <span class="text-emerald-400 font-semibold">Java {reqJava} Ready</span>
              {:else}
                <span class="w-2 h-2 rounded-full bg-amber-400 animate-pulse"></span>
                <span class="text-amber-400 font-semibold">Auto-Installs on Launch</span>
              {/if}
            </span>
          </div>
        </div>
      </div>

      <button
        onclick={() => (launcher.activeTab = "settings")}
        class="mt-5 w-full py-2.5 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 text-white text-xs font-bold uppercase tracking-wider flex items-center justify-center gap-1.5 transition-all cursor-pointer"
      >
        <span>Configure Engine</span>
        <ArrowUpRight class="w-4 h-4" />
      </button>
    </div>

    <!-- World Saves & 1-Click Backups Card -->
    <div
      class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 hover:border-white/16 transition-all flex flex-col justify-between group"
    >
      <div>
        <div class="flex items-center justify-between mb-3">
          <div
            class="w-10 h-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400 group-hover:scale-110 transition-transform"
          >
            <Map class="w-5 h-5" />
          </div>
          <span
            class="text-xs font-bold uppercase tracking-wider px-2.5 py-0.5 rounded bg-emerald-500/15 text-emerald-400 border border-emerald-500/20"
          >
            Saves & Data
          </span>
        </div>
        <h3 class="font-bold text-base text-white">World Saves & Backups</h3>
        <p class="text-xs text-neutral-300 mt-1.5 leading-relaxed font-normal">
          Inspect singleplayer worlds, calculate folder size, and generate instant 1-click .zip backups safely into backups.
        </p>
      </div>

      <button
        onclick={() => {
          showWorldsModal = true;
          launcher.loadWorlds();
        }}
        disabled={!launcher.activeInstance}
        class="mt-5 w-full py-2.5 rounded-xl bg-white/5 hover:bg-emerald-500/20 hover:border-emerald-500/30 border border-white/10 text-white text-xs font-bold uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer disabled:opacity-50"
      >
        <Archive class="w-4 h-4 text-emerald-400" />
        <span>Manage World Saves</span>
      </button>
    </div>

    <!-- In-Game Screenshot Gallery Card -->
    <div
      class="p-6 rounded-2xl bg-[#0e1017]/90 border border-white/8 hover:border-white/16 transition-all flex flex-col justify-between group"
    >
      <div>
        <div class="flex items-center justify-between mb-3">
          <div
            class="w-10 h-10 rounded-xl bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400 group-hover:scale-110 transition-transform"
          >
            <Camera class="w-5 h-5" />
          </div>
          <span
            class="text-xs font-bold uppercase tracking-wider px-2.5 py-0.5 rounded bg-amber-500/15 text-amber-400 border border-amber-500/20"
          >
            Memories
          </span>
        </div>
        <h3 class="font-bold text-base text-white">In-Game Screenshot Gallery</h3>
        <p class="text-xs text-neutral-300 mt-1.5 leading-relaxed font-normal">
          Browse full-resolution F2 captures, view thumbnails in-launcher, and preview screenshots in high definition.
        </p>
      </div>

      <button
        onclick={() => {
          showScreenshotsModal = true;
          launcher.loadScreenshots();
        }}
        disabled={!launcher.activeInstance}
        class="mt-5 w-full py-2.5 rounded-xl bg-white/5 hover:bg-amber-500/20 hover:border-amber-500/30 border border-white/10 text-white text-xs font-bold uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer disabled:opacity-50"
      >
        <Eye class="w-4 h-4 text-amber-400" />
        <span>View Screenshots</span>
      </button>
    </div>
  </div>

  <!-- World Saves & Backups Modal -->
  {#if showWorldsModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md animate-in fade-in">
      <div class="w-full max-w-2xl max-h-[85vh] p-6 rounded-3xl bg-[#0f111a] border border-white/15 shadow-2xl flex flex-col space-y-4">
        <div class="flex items-center justify-between pb-3 border-b border-white/10 shrink-0">
          <div class="flex items-center gap-2.5">
            <div class="w-9 h-9 rounded-xl bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400">
              <Map class="w-5 h-5" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white">Singleplayer World Saves</h3>
              <p class="text-xs text-neutral-400">Manage worlds and create instant 1-click .zip backups</p>
            </div>
          </div>
          <button
            onclick={() => (showWorldsModal = false)}
            class="p-2 rounded-xl hover:bg-white/10 text-neutral-400 hover:text-white transition-all cursor-pointer"
          >
            <X class="w-4 h-4" />
          </button>
        </div>

        <div class="flex-1 overflow-y-auto space-y-3 min-h-[220px]">
          {#if launcher.isLoadingWorlds}
            <div class="py-12 flex flex-col items-center justify-center gap-2.5 text-neutral-400">
              <RefreshCw class="w-6 h-6 animate-spin text-emerald-400" />
              <span class="text-xs">Scanning saves folder...</span>
            </div>
          {:else if launcher.worlds.length === 0}
            <div class="py-12 text-center text-xs text-neutral-400 space-y-2">
              <Map class="w-8 h-8 mx-auto text-neutral-500" />
              <p>No world saves found in this profile yet.</p>
              <p class="text-neutral-500">Launch Minecraft and start a world to see it here!</p>
            </div>
          {:else}
            {#each launcher.worlds as world}
              <div class="p-4 rounded-2xl bg-white/[0.03] border border-white/8 hover:border-white/16 transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <div class="flex items-center gap-3 min-w-0">
                  {#if world.icon_base64}
                    <img src={world.icon_base64} alt={world.name} class="w-12 h-12 rounded-xl object-cover border border-white/10 shrink-0" />
                  {:else}
                    <div class="w-12 h-12 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center text-neutral-400 shrink-0">
                      <Map class="w-6 h-6" />
                    </div>
                  {/if}
                  <div class="min-w-0">
                    <h4 class="text-sm font-bold text-white truncate">{world.name}</h4>
                    <div class="flex items-center gap-2 text-[11px] font-mono text-neutral-400 mt-0.5">
                      <span>Folder: {world.folder_name}</span>
                      <span>•</span>
                      <span>{(world.size_bytes / (1024 * 1024)).toFixed(1)} MB</span>
                    </div>
                  </div>
                </div>

                <div class="flex items-center gap-2 shrink-0">
                  <button
                    onclick={() => launcher.backupWorld(world.folder_name)}
                    disabled={world.isBackingUp}
                    title="Create zip backup in backups directory"
                    class="px-3 py-1.5 rounded-xl bg-emerald-500/20 hover:bg-emerald-500/30 border border-emerald-500/30 text-emerald-300 text-xs font-bold flex items-center gap-1.5 transition-all cursor-pointer disabled:opacity-50"
                  >
                    {#if world.isBackingUp}
                      <div class="w-3.5 h-3.5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                      <span>Zipping...</span>
                    {:else}
                      <Archive class="w-3.5 h-3.5" />
                      <span>Backup .zip</span>
                    {/if}
                  </button>

                  <button
                    onclick={() => launcher.deleteWorld(world.folder_name)}
                    title="Delete save folder"
                    class="p-2 rounded-xl bg-white/5 hover:bg-rose-500/20 text-neutral-400 hover:text-rose-300 border border-white/10 transition-all cursor-pointer"
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            {/each}
          {/if}
        </div>

        <div class="pt-3 border-t border-white/10 flex flex-wrap justify-between items-center gap-2 shrink-0">
          <span class="text-xs text-neutral-400 font-mono">{launcher.worlds.length} worlds detected</span>
          <div class="flex items-center gap-2">
            <button
              onclick={() => launcher.importWorld()}
              class="px-3 py-1.5 rounded-xl bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-300 text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer border border-emerald-500/30"
              title="Import a world save archive (.zip)"
            >
              <Download class="w-3.5 h-3.5" />
              <span>Import .zip</span>
            </button>
            <button
              onclick={() => launcher.importVanillaWorlds()}
              class="px-3 py-1.5 rounded-xl bg-white/5 hover:bg-white/10 text-neutral-300 hover:text-white text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer border border-white/10"
              title="Import worlds from Vanilla Minecraft (%APPDATA%/.minecraft/saves)"
            >
              <Sparkles class="w-3.5 h-3.5 text-amber-400" />
              <span>Import Vanilla</span>
            </button>
            <button
              onclick={() => launcher.openInstanceSaves()}
              class="px-3.5 py-1.5 rounded-xl bg-white/5 hover:bg-white/10 text-neutral-200 hover:text-white text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer border border-white/10"
            >
              <FolderOpen class="w-3.5 h-3.5" />
              <span>Open Saves Folder</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Screenshot Gallery Modal -->
  {#if showScreenshotsModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md animate-in fade-in">
      <div class="w-full max-w-4xl max-h-[90vh] p-6 rounded-3xl bg-[#0f111a] border border-white/15 shadow-2xl flex flex-col space-y-4">
        <div class="flex items-center justify-between pb-3 border-b border-white/10 shrink-0">
          <div class="flex items-center gap-2.5">
            <div class="w-9 h-9 rounded-xl bg-amber-500/20 border border-amber-500/30 flex items-center justify-center text-amber-400">
              <Camera class="w-5 h-5" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white">In-Game Screenshot Gallery</h3>
              <p class="text-xs text-neutral-400">View and inspect F2 captures from your gameplay</p>
            </div>
          </div>
          <button
            onclick={() => (showScreenshotsModal = false)}
            class="p-2 rounded-xl hover:bg-white/10 text-neutral-400 hover:text-white transition-all cursor-pointer"
          >
            <X class="w-4 h-4" />
          </button>
        </div>

        <div class="flex-1 overflow-y-auto min-h-[300px]">
          {#if launcher.isLoadingScreenshots}
            <div class="py-16 flex flex-col items-center justify-center gap-2.5 text-neutral-400">
              <RefreshCw class="w-6 h-6 animate-spin text-amber-400" />
              <span class="text-xs">Loading screenshots...</span>
            </div>
          {:else if launcher.screenshots.length === 0}
            <div class="py-16 text-center text-xs text-neutral-400 space-y-2">
              <Camera class="w-10 h-10 mx-auto text-neutral-500" />
              <p>No screenshots taken yet in this profile.</p>
              <p class="text-neutral-500">Press <strong>F2</strong> inside Minecraft to take a screenshot!</p>
            </div>
          {:else}
            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
              {#each launcher.screenshots as shot}
                <div class="group relative rounded-2xl overflow-hidden bg-black/40 border border-white/10 hover:border-amber-400/40 transition-all flex flex-col">
                  {#if shot.thumbnail_base64}
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      onclick={() => (selectedPreviewImage = shot.thumbnail_base64)}
                      class="relative aspect-video overflow-hidden cursor-zoom-in"
                    >
                      <img
                        src={shot.thumbnail_base64}
                        alt={shot.filename}
                        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-200"
                      />
                      <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors flex items-center justify-center opacity-0 group-hover:opacity-100">
                        <span class="px-2 py-1 rounded bg-black/70 text-white text-[10px] font-bold">Zoom</span>
                      </div>
                    </div>
                  {:else}
                    <div class="aspect-video bg-white/5 flex items-center justify-center text-neutral-400">
                      <Image class="w-8 h-8" />
                    </div>
                  {/if}
                  <div class="p-2.5 flex items-center justify-between text-[11px] font-mono text-neutral-400">
                    <span class="truncate max-w-[120px]" title={shot.filename}>{shot.filename}</span>
                    <button
                      onclick={() => launcher.deleteScreenshot(shot.filename)}
                      title="Delete screenshot"
                      class="p-1 rounded text-neutral-400 hover:text-rose-400 transition-colors"
                    >
                      <Trash2 class="w-3.5 h-3.5" />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <div class="pt-3 border-t border-white/10 flex justify-between items-center shrink-0">
          <span class="text-xs text-neutral-400 font-mono">{launcher.screenshots.length} screenshots found</span>
          <button
            onclick={() => launcher.openInstanceScreenshots()}
            class="px-3.5 py-1.5 rounded-xl bg-white/5 hover:bg-white/10 text-neutral-200 hover:text-white text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer border border-white/10"
          >
            <FolderOpen class="w-3.5 h-3.5" />
            <span>Open Screenshots Folder</span>
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Fullscreen Image Lightbox Preview -->
  {#if selectedPreviewImage}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onclick={() => (selectedPreviewImage = null)}
      class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-8 bg-black/95 backdrop-blur-lg animate-in fade-in cursor-zoom-out"
    >
      <img
        src={selectedPreviewImage}
        alt="Screenshot Fullscreen Preview"
        class="max-w-full max-h-full rounded-2xl object-contain shadow-2xl border border-white/10"
      />
      <button
        onclick={() => (selectedPreviewImage = null)}
        class="absolute top-6 right-6 p-2 rounded-xl bg-white/10 hover:bg-white/20 text-white transition-all cursor-pointer"
      >
        <X class="w-6 h-6" />
      </button>
    </div>
  {/if}
</div>
