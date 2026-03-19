<script lang="ts">
  import { navigate } from "../lib/stores/ui.svelte";
  import { repos as reposApi, odeSimulator } from "../lib/api/tauri";
  import type { Repo } from "../lib/api/tauri";

  let recentRepos = $state<Repo[]>([]);
  let seedingDemos = $state(false);

  async function loadRepos() {
    try {
      recentRepos = await reposApi.list();
    } catch {}
  }

  async function seedDemos() {
    seedingDemos = true;
    try {
      await odeSimulator.seedBacterialDemo();
      await loadRepos();
    } catch {}
    seedingDemos = false;
  }

  $effect(() => {
    loadRepos();
  });
</script>

<div class="flex h-full flex-col items-center justify-center gap-8 p-8">
  <div class="text-center">
    <h1 class="mb-2 text-4xl font-bold text-base-100">
      <span class="text-accent">Plasma</span>geddon
    </h1>
    <p class="text-base-400">End of bad designs</p>
  </div>

  <div class="grid w-full max-w-3xl grid-cols-3 gap-4">
    <button
      onclick={() => navigate("/editor")}
      class="flex flex-col items-center gap-3 rounded-xl border border-base-600 bg-base-800 p-6 transition-colors hover:border-accent/50 hover:bg-base-700"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-accent">
        <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
      <span class="font-medium text-base-100">New Construct</span>
      <span class="text-xs text-base-400">Start from scratch</span>
    </button>

    <button
      onclick={() => navigate("/repos")}
      class="flex flex-col items-center gap-3 rounded-xl border border-base-600 bg-base-800 p-6 transition-colors hover:border-accent/50 hover:bg-base-700"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-accent">
        <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
      </svg>
      <span class="font-medium text-base-100">Browse Repos</span>
      <span class="text-xs text-base-400">Open existing projects</span>
    </button>

    <button
      onclick={() => navigate("/notebook")}
      class="flex flex-col items-center gap-3 rounded-xl border border-base-600 bg-base-800 p-6 transition-colors hover:border-accent/50 hover:bg-base-700"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-accent">
        <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
      </svg>
      <span class="font-medium text-base-100">Lab Notebook</span>
      <span class="text-xs text-base-400">Document your work</span>
    </button>

    <button
      onclick={() => { navigate("/editor"); }}
      class="flex flex-col items-center gap-3 rounded-xl border border-base-600 bg-base-800 p-6 transition-colors hover:border-accent/50 hover:bg-base-700"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-accent">
        <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
      </svg>
      <span class="font-medium text-base-100">Import File</span>
      <span class="text-xs text-base-400">GenBank, FASTA, .dna</span>
    </button>

    <button
      onclick={seedDemos}
      disabled={seedingDemos}
      class="flex flex-col items-center gap-3 rounded-xl border border-green-600/30 bg-base-800 p-6 transition-colors hover:border-green-500/50 hover:bg-base-700 disabled:opacity-50"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-green-400">
        <circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>
      </svg>
      <span class="font-medium text-base-100">{seedingDemos ? "Seeding..." : "Bacterial Demos"}</span>
      <span class="text-xs text-base-400">Repressilator, Toggle Switch</span>
    </button>

    <button
      onclick={() => navigate("/settings")}
      class="flex flex-col items-center gap-3 rounded-xl border border-base-600 bg-base-800 p-6 transition-colors hover:border-accent/50 hover:bg-base-700"
    >
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-accent">
        <circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 012.83-2.83l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 2.83l-.06.06A1.65 1.65 0 0019.4 9c.26.46.77.77 1.33.82H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
      </svg>
      <span class="font-medium text-base-100">Settings</span>
      <span class="text-xs text-base-400">API keys & preferences</span>
    </button>
  </div>

  {#if recentRepos.length > 0}
    <div class="w-full max-w-3xl">
      <h2 class="mb-3 text-sm font-semibold text-base-300">Recent Repos</h2>
      <div class="space-y-1">
        {#each recentRepos.slice(0, 5) as repo}
          <button
            onclick={() => navigate("/repos")}
            class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors hover:bg-base-800"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-base-400">
              <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
            </svg>
            <span class="text-sm text-base-200">{repo.name}</span>
            <span class="text-xs text-base-500 truncate">{repo.description}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
