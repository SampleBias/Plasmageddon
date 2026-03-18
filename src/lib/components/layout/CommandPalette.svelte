<script lang="ts">
  import { showCommandPalette, navigate } from "../../stores/ui.svelte";
  import { constructs as constructsApi, parts as partsApi } from "../../api/tauri";
  import Fuse from "fuse.js";

  interface PaletteItem {
    id: string;
    label: string;
    category: string;
    action: () => void;
  }

  let query = $state("");
  let items = $state<PaletteItem[]>([]);
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement;

  const staticCommands: PaletteItem[] = [
    { id: "nav-home", label: "Go to Home", category: "Navigation", action: () => go("/") },
    { id: "nav-repos", label: "Go to Repos", category: "Navigation", action: () => go("/repos") },
    { id: "nav-editor", label: "Go to Editor", category: "Navigation", action: () => go("/editor") },
    { id: "nav-settings", label: "Go to Settings", category: "Navigation", action: () => go("/settings") },
    { id: "new-construct", label: "New Construct", category: "Actions", action: () => go("/editor") },
  ];

  function go(path: string) {
    navigate(path);
    showCommandPalette.value = false;
  }

  let fuse = new Fuse(staticCommands, {
    keys: ["label", "category"],
    threshold: 0.4,
  });

  $effect(() => {
    if (query.trim() === "") {
      items = staticCommands;
    } else {
      const results = fuse.search(query);
      items = results.map((r) => r.item);
    }
    selectedIndex = 0;
  });

  $effect(() => {
    if (showCommandPalette.value && inputEl) {
      inputEl.focus();
    }
  });

  async function searchBackend() {
    if (query.length < 2) return;
    try {
      const [foundConstructs, foundParts] = await Promise.all([
        constructsApi.search(query),
        partsApi.search(query),
      ]);

      const cItems: PaletteItem[] = foundConstructs.map((c) => ({
        id: `c-${c.id}`,
        label: `${c.name} (${c.sequence.length} bp)`,
        category: "Constructs",
        action: () => go(`/editor?id=${c.id}`),
      }));

      const pItems: PaletteItem[] = foundParts.map((p) => ({
        id: `p-${p.id}`,
        label: `${p.name} [${p.part_type}]`,
        category: "Parts",
        action: () => {},
      }));

      items = [...items, ...cItems, ...pItems];
    } catch {
      // Backend not available yet
    }
  }

  $effect(() => {
    if (query.length >= 2) {
      searchBackend();
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === "Enter" && items[selectedIndex]) {
      e.preventDefault();
      items[selectedIndex].action();
      showCommandPalette.value = false;
    } else if (e.key === "Escape") {
      showCommandPalette.value = false;
    }
  }

  function handleOverlayClick() {
    showCommandPalette.value = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-start justify-center bg-black/60 pt-[15vh]"
  onkeydown={handleKeydown}
  onclick={handleOverlayClick}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="w-full max-w-lg rounded-xl border border-base-600 bg-base-800 shadow-2xl overflow-hidden"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center border-b border-base-700 px-4 py-3">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-base-400 mr-3">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        bind:this={inputEl}
        bind:value={query}
        placeholder="Search constructs, parts, or type a command..."
        class="flex-1 bg-transparent text-sm text-base-100 outline-none placeholder:text-base-500"
      />
    </div>

    <div class="max-h-80 overflow-y-auto p-1">
      {#if items.length === 0}
        <div class="px-4 py-6 text-center text-sm text-base-400">No results found</div>
      {:else}
        {#each items as item, i}
          <button
            class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm transition-colors
              {i === selectedIndex ? 'bg-accent/20 text-accent' : 'text-base-200 hover:bg-base-700'}"
            onclick={() => { item.action(); showCommandPalette.value = false; }}
            onmouseenter={() => selectedIndex = i}
          >
            <span class="rounded bg-base-600 px-1.5 py-0.5 text-xs text-base-400">{item.category}</span>
            <span>{item.label}</span>
          </button>
        {/each}
      {/if}
    </div>
  </div>
</div>
