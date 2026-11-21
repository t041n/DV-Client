<script lang="ts">
  import { launcher } from "../../state/launcher.svelte";
  import { 
    ChevronDown, 
    User, 
    ShieldCheck, 
    HardDrive, 
    Plus, 
    Zap,
    Box,
    Sparkles
  } from "lucide-svelte";

  let showInstanceDropdown = $state(false);

  function getLoaderColor(loader: string) {
    const l = loader.toLowerCase();
    if (l.includes("fabric")) return "text-purple-300 bg-purple-500/15 border-purple-500/30";
    if (l.includes("neoforge")) return "text-orange-300 bg-orange-500/15 border-orange-500/30";
    if (l.includes("forge")) return "text-amber-300 bg-amber-500/15 border-amber-500/30";
    return "text-neutral-300 bg-neutral-500/15 border-neutral-500/30";
  }

  function getAvatarUrl() {
    if (!launcher.activeAccount) return null;
    if (launcher.activeAccount.avatar_url) return launcher.activeAccount.avatar_url;
    if (launcher.activeAccount.account_type === "microsoft" && launcher.activeAccount.id) {
      return `https://crafthead.net/avatar/${launcher.activeAccount.id}`;
    }
    return `https://crafthead.net/avatar/${launcher.activeAccount.username}`;
  }
</script>

<header class="h-14 sm:h-16 border-b border-white/8 bg-[#0a0c12]/90 backdrop-blur-xl px-2 sm:px-6 flex items-center justify-between gap-2 shrink-0 z-30 select-none">
  <!-- Left: Active Profile Quick Selector -->
  <div class="relative min-w-0">
    <button
      onclick={() => showInstanceDropdown = !showInstanceDropdown}
      class="flex items-center gap-2 sm:gap-3 px-2.5 sm:px-3.5 py-1.5 sm:py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-all cursor-pointer group max-w-[150px] sm:max-w-[240px] md:max-w-none"
    >
      <div class="w-7 h-7 sm:w-8 sm:h-8 rounded-lg bg-red-500/15 border border-red-500/30 flex items-center justify-center text-red-400 group-hover:scale-105 transition-transform shrink-0">
        <Box class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
      </div>

      <div class="text-left min-w-0">
        <div class="flex items-center gap-1.5 sm:gap-2">
          <span class="text-xs sm:text-sm font-bold text-white tracking-tight truncate">
            {launcher.activeInstance?.name || "No Profile"}
          </span>
          {#if launcher.activeInstance}
            <span class="text-[9px] sm:text-[11px] px-1 sm:px-2 py-0.5 rounded border {getLoaderColor(launcher.activeInstance.mod_loader)} uppercase font-semibold shrink-0">
              {launcher.activeInstance.mod_loader}
            </span>
          {/if}
        </div>
        <div class="text-[10px] sm:text-xs text-neutral-300 font-medium truncate hidden sm:block">
          {launcher.activeInstance ? `Minecraft ${launcher.activeInstance.mc_version}` : "Select a profile"}
        </div>
      </div>

      <ChevronDown class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-neutral-400 group-hover:text-white transition-transform shrink-0 {showInstanceDropdown ? 'rotate-180' : ''}" />
    </button>

    <!-- Dropdown Menu -->
    {#if showInstanceDropdown}
      <div 
        class="absolute left-0 mt-2 w-72 sm:w-76 rounded-2xl bg-[#0f121b] border border-white/12 shadow-2xl p-2.5 z-50 backdrop-blur-3xl animate-in fade-in slide-in-from-top-2 duration-150"
      >
        <div class="px-3 py-2 text-xs font-bold uppercase tracking-wider text-neutral-400 flex items-center justify-between border-b border-white/8 mb-1.5">
          <span>Switch Profile</span>
          <button 
            onclick={() => { showInstanceDropdown = false; launcher.showCreateInstanceModal = true; }}
            class="text-red-400 hover:text-red-300 flex items-center gap-1 cursor-pointer font-semibold"
          >
            <Plus class="w-3.5 h-3.5" /> New Profile
          </button>
        </div>

        <div class="max-h-64 overflow-y-auto space-y-1">
          {#each launcher.instances as inst}
            <button
              onclick={() => {
                launcher.selectInstance(inst.id);
                showInstanceDropdown = false;
              }}
              class="w-full flex items-center justify-between p-2.5 rounded-xl text-left transition-all cursor-pointer {inst.is_active ? 'bg-red-500/15 border border-red-500/30 text-white' : 'hover:bg-white/5 text-neutral-200'}"
            >
              <div class="truncate">
                <div class="text-sm font-semibold truncate">{inst.name}</div>
                <div class="text-xs text-neutral-400">MC {inst.mc_version} • {inst.mod_loader}</div>
              </div>
              {#if inst.is_active}
                <span class="w-2.5 h-2.5 rounded-full bg-red-400 shrink-0"></span>
              {/if}
            </button>
          {/each}

          {#if launcher.instances.length === 0}
            <div class="p-4 text-center text-xs text-neutral-400">
              No profiles found. Create your first profile!
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <!-- Center: Performance Engine Indicator -->
  <div class="hidden lg:flex items-center gap-3 px-4 py-2 rounded-full bg-white/4 border border-white/8 text-xs text-neutral-300">
    <div class="flex items-center gap-1.5 text-red-400 font-bold uppercase tracking-wide">
      <Zap class="w-4 h-4 fill-red-400/20" />
      <span>DV TURBO ENGINE</span>
    </div>
    <span class="text-neutral-600">•</span>
    <div class="flex items-center gap-1.5 text-neutral-200 font-medium">
      <HardDrive class="w-3.5 h-3.5 text-neutral-400" />
      <span class="font-mono">{launcher.settings.default_max_memory_mb || 4096} MB RAM</span>
    </div>
  </div>

  <!-- Right: Account Profile Pill & Update Pill -->
  <div class="flex items-center gap-2 sm:gap-3 shrink-0">
    {#if launcher.updateInfo?.has_update}
      <button
        onclick={() => (launcher.showUpdateModal = true)}
        class="flex items-center gap-1.5 sm:gap-2 px-2.5 sm:px-3 py-1.5 rounded-xl bg-gradient-to-r from-emerald-500/20 to-teal-500/20 hover:from-emerald-500/30 hover:to-teal-500/30 border border-emerald-500/40 text-emerald-300 text-xs font-bold transition-all cursor-pointer shadow-lg shadow-emerald-500/10 animate-pulse hover:animate-none"
        title="New version available: {launcher.updateInfo.latest_version}"
      >
        <Sparkles class="w-3.5 h-3.5 text-emerald-400 shrink-0" />
        <span class="hidden md:inline">Update</span>
        <span class="text-white font-mono bg-emerald-500/30 px-1.5 py-0.5 rounded text-[10px] sm:text-[11px] font-bold">{launcher.updateInfo.latest_version}</span>
      </button>
    {/if}

    <button
      onclick={() => launcher.showAccountModal = true}
      class="flex items-center gap-2 sm:gap-3 px-2.5 sm:px-3.5 py-1.5 sm:py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-all cursor-pointer group max-w-[140px] sm:max-w-[200px] md:max-w-none"
    >
      {#if launcher.activeAccount}
        <div class="relative w-7 h-7 sm:w-8 sm:h-8 rounded-lg overflow-hidden bg-black/40 border border-white/10 shrink-0">
          <img 
            src={getAvatarUrl()} 
            alt={launcher.activeAccount.username}
            class="w-full h-full object-cover rendering-pixelated"
            onerror={(e) => {
              (e.currentTarget as HTMLElement).style.display = 'none';
            }}
          />
        </div>
        <div class="text-left min-w-0">
          <div class="flex items-center gap-1 sm:gap-1.5">
            <span class="text-xs sm:text-sm font-bold text-white tracking-tight truncate">{launcher.activeAccount.username}</span>
            {#if launcher.activeAccount.account_type === "microsoft"}
              <ShieldCheck class="w-3.5 h-3.5 text-sky-400 shrink-0" />
            {/if}
          </div>
          <div class="text-[10px] sm:text-xs text-neutral-400 font-medium truncate hidden sm:block">
            {launcher.activeAccount.account_type === "microsoft" ? "Microsoft Account" : "Offline Player"}
          </div>
        </div>
      {:else}
        <div class="w-7 h-7 sm:w-8 sm:h-8 rounded-lg bg-red-500/15 border border-red-500/30 flex items-center justify-center text-red-400 shrink-0">
          <User class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
        </div>
        <div class="text-left min-w-0">
          <div class="text-xs sm:text-sm font-bold text-white truncate">Choose Account</div>
          <div class="text-[10px] sm:text-xs text-red-400 font-semibold truncate hidden sm:block">Required</div>
        </div>
      {/if}
      <ChevronDown class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-neutral-400 group-hover:text-white transition-colors shrink-0" />
    </button>
  </div>
</header>
