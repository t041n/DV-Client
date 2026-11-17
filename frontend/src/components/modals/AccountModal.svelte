<script lang="ts">
  import { launcher } from "../../state/launcher.svelte";
  import { X, User, ShieldCheck, ExternalLink, Trash2 } from "lucide-svelte";

  let offlineName = $state("");
  let msCode = $state("");

  function handleAddOffline() {
    if (!offlineName.trim()) return;
    launcher.addOfflineAccount(offlineName.trim());
    offlineName = "";
  }

  function handleCompleteMs() {
    if (!msCode.trim()) return;
    launcher.completeMicrosoftLogin(msCode.trim());
    msCode = "";
  }

  function getAvatar(acc: any) {
    if (acc.avatar_url) return acc.avatar_url;
    if (acc.account_type === "microsoft" && acc.id) {
      return `https://crafthead.net/avatar/${acc.id}`;
    }
    return `https://crafthead.net/avatar/${acc.username}`;
  }
</script>

{#if launcher.showAccountModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md animate-in fade-in duration-200"
  >
    <div
      class="relative w-full max-w-lg rounded-3xl bg-[#0e1017] border border-white/10 shadow-2xl p-6 space-y-6"
    >
      <!-- Modal Header -->
      <div
        class="flex items-center justify-between pb-4 border-b border-white/8"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-xl bg-red-500/10 border border-red-500/20 flex items-center justify-center text-red-500"
          >
            <User class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-base font-bold uppercase text-white">
              Player Accounts
            </h3>
            <p class="text-xs text-neutral-300">
              Switch or add Microsoft and Offline accounts
            </p>
          </div>
        </div>

        <button
          onclick={() => (launcher.showAccountModal = false)}
          class="p-2 rounded-xl bg-white/5 hover:bg-white/10 text-neutral-300 hover:text-white transition-all cursor-pointer"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Account List -->
      <div class="space-y-2 max-h-56 overflow-y-auto pr-1">
        {#each launcher.accounts as acc}
          <div
            class="p-3.5 rounded-2xl border transition-all flex items-center justify-between {acc.is_active
              ? 'bg-red-500/10 border-red-500/30'
              : 'bg-white/4 border-white/8 hover:bg-white/6'}"
          >
            <div class="flex items-center gap-3.5 min-w-0">
              <img
                src={getAvatar(acc)}
                alt={acc.username}
                class="w-10 h-10 rounded-xl object-cover bg-black/40 border border-white/10 shrink-0"
                onerror={(e) => {
                  (e.currentTarget as HTMLElement).style.display = "none";
                }}
              />
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="text-sm font-bold text-white truncate"
                    >{acc.username}</span
                  >
                  {#if acc.is_active}
                    <span
                      class="px-2 py-0.5 rounded bg-red-500 text-white text-[10px] font-black uppercase"
                    >
                      Active
                    </span>
                  {/if}
                </div>
                <div class="text-xs text-neutral-400">
                  {acc.account_type === "microsoft"
                    ? "Microsoft Account (Online)"
                    : "Normal Offline Account"}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-2">
              {#if !acc.is_active}
                <button
                  onclick={() => launcher.selectAccount(acc.id)}
                  class="px-3.5 py-1.5 rounded-lg bg-white/6 hover:bg-white/12 text-white text-xs font-bold transition-all cursor-pointer"
                >
                  Select
                </button>
              {/if}

              <button
                onclick={() => launcher.logoutAccount(acc.id)}
                title="Remove account"
                class="p-2 rounded-lg bg-white/5 hover:bg-red-500/20 text-neutral-400 hover:text-red-400 transition-all cursor-pointer"
              >
                <Trash2 class="w-4 h-4" />
              </button>
            </div>
          </div>
        {/each}

        {#if launcher.accounts.length === 0}
          <div class="p-4 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-center text-xs text-amber-200 space-y-1">
            <div class="font-bold">No Player Account Selected</div>
            <div class="text-neutral-400">Please choose a username below (offline or Microsoft) to play.</div>
          </div>
        {/if}
      </div>

      <!-- Add Accounts Tabbed / Stacked Section -->
      <div class="space-y-4 pt-4 border-t border-white/8">
        <!-- Add Offline Account (Normal) -->
        <div class="p-4 rounded-2xl bg-white/4 border border-white/8 space-y-3">
          <div class="flex items-center justify-between">
            <span
              class="text-xs font-bold text-white uppercase tracking-wide flex items-center gap-2"
            >
              <User class="w-4 h-4 text-red-400" />
              <span>Normal Offline Account (Singleplayer & SMPs)</span>
            </span>
          </div>
          <p class="text-[11px] text-neutral-400 leading-relaxed">
            No password or email required. Jump straight into singleplayer worlds, LAN, and offline SMP servers.
          </p>

          <div class="flex gap-2">
            <input
              type="text"
              bind:value={offlineName}
              onkeydown={(e) => e.key === "Enter" && handleAddOffline()}
              placeholder="Enter your username (e.g. Alex, Torin, Steve)..."
              class="flex-1 px-3.5 py-2.5 rounded-xl bg-black/40 border border-white/10 text-xs text-white outline-none focus:border-red-500/50"
            />
            <button
              onclick={handleAddOffline}
              class="px-5 py-2.5 rounded-xl bg-red-600 hover:bg-red-500 text-white font-bold text-xs uppercase cursor-pointer transition-all shadow-md active:scale-95 shrink-0"
            >
              Set Username
            </button>
          </div>
        </div>

        <!-- Microsoft OAuth Login (Online) -->
        <div class="p-4 rounded-2xl bg-white/4 border border-white/8 space-y-3">
          <div class="flex items-center justify-between">
            <span
              class="text-xs font-bold text-white uppercase tracking-wide flex items-center gap-2"
            >
              <ShieldCheck class="w-4 h-4 text-sky-400" />
              <span>Microsoft Account (Online / Realms / Hypixel)</span>
            </span>
          </div>
          <p class="text-[11px] text-neutral-400 leading-relaxed">
            Connect an official Minecraft account to play on verified online-mode servers.
          </p>

          {#if !launcher.isMsLoggingIn}
            <button
              onclick={() => launcher.startMicrosoftLogin()}
              class="w-full py-2.5 rounded-xl bg-sky-600 hover:bg-sky-500 text-white font-bold text-xs uppercase tracking-wider flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md active:scale-95"
            >
              <ExternalLink class="w-4 h-4" />
              <span>Sign in with Microsoft</span>
            </button>
          {:else}
            <div class="space-y-2">
              <p class="text-xs text-neutral-300">
                Log into Microsoft in your browser, then copy the returned code
                or URL and paste it below:
              </p>
              <div class="flex gap-2">
                <input
                  type="text"
                  bind:value={msCode}
                  placeholder="Paste Microsoft auth code here..."
                  class="flex-1 px-3.5 py-2 rounded-xl bg-black/40 border border-white/10 text-xs font-mono text-white outline-none focus:border-sky-500"
                />
                <button
                  onclick={handleCompleteMs}
                  class="px-4 py-2 rounded-xl bg-sky-600 hover:bg-sky-500 text-white font-bold text-xs uppercase cursor-pointer"
                >
                  Verify
                </button>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
