<script lang="ts">
  import { showCommandPalette } from "../../stores/ui.svelte";
  import { currentConstruct } from "../../stores/construct.svelte";
  import { navigate } from "../../stores/ui.svelte";

  function openPalette() {
    showCommandPalette.value = true;
  }
</script>

<header class="flex h-12 items-center justify-between border-b border-base-700 bg-base-800 px-4">
  <div class="flex items-center gap-3">
    <button onclick={() => navigate("/")} class="flex items-center gap-2 text-accent hover:text-accent-hover transition-colors">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10" />
        <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
        <line x1="2" y1="12" x2="22" y2="12" />
      </svg>
      <span class="font-bold text-sm tracking-wide">PLASMAGEDDON</span>
    </button>

    {#if currentConstruct.value}
      <span class="text-base-400">|</span>
      <span class="text-sm text-base-200">{currentConstruct.value.name}</span>
      <span class="rounded bg-base-600 px-1.5 py-0.5 text-xs text-base-300">
        {currentConstruct.value.topology}
      </span>
      <span class="text-xs text-base-400">
        {currentConstruct.value.sequence.length.toLocaleString()} bp
      </span>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    <button
      onclick={openPalette}
      class="flex items-center gap-2 rounded-md border border-base-600 bg-base-700 px-3 py-1 text-sm text-base-300 hover:bg-base-600 hover:text-base-100 transition-colors"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <span>Search</span>
      <kbd class="rounded border border-base-500 bg-base-800 px-1 py-0.5 text-xs text-base-400">Ctrl+K</kbd>
    </button>

    <button
      onclick={() => navigate("/editor")}
      class="rounded-md bg-accent px-3 py-1 text-sm font-medium text-white hover:bg-accent-hover transition-colors"
    >
      + New
    </button>
  </div>
</header>
