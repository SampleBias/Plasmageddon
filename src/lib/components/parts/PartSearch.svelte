<script lang="ts">
  import { parts as partsApi } from "../../api/tauri";
  import type { Part } from "../../stores/parts.svelte";

  interface Props {
    onselect: (part: Part) => void;
  }

  let { onselect }: Props = $props();
  let query = $state("");
  let results = $state<Part[]>([]);

  async function search() {
    if (query.trim().length < 2) {
      results = [];
      return;
    }
    try {
      results = await partsApi.search(query);
    } catch {
      results = [];
    }
  }

  $effect(() => {
    query;
    search();
  });
</script>

<div>
  <input
    bind:value={query}
    placeholder="Find a part..."
    class="w-full rounded-md border border-base-600 bg-base-700 px-2.5 py-1.5 text-xs text-base-100 outline-none focus:border-accent"
  />
  {#if results.length > 0}
    <div class="mt-1 max-h-48 overflow-y-auto rounded-md border border-base-600 bg-base-800">
      {#each results as part}
        <button
          onclick={() => onselect(part)}
          class="flex w-full items-center gap-2 px-3 py-1.5 text-left text-xs hover:bg-base-700"
        >
          <span class="text-base-200">{part.name}</span>
          <span class="text-base-500">[{part.part_type}]</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
