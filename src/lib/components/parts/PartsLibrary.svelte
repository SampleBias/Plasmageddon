<script lang="ts">
  import { partsList, partsFilter, partsTypeFilter, partsLoading } from "../../stores/parts.svelte";
  import { parts as partsApi } from "../../api/tauri";
  import PartCard from "./PartCard.svelte";

  const partTypes = [
    { id: "", label: "All" },
    { id: "promoter", label: "Promoters" },
    { id: "cds", label: "CDS" },
    { id: "terminator", label: "Terminators" },
    { id: "ori", label: "Origins" },
    { id: "marker", label: "Markers" },
    { id: "tag", label: "Tags" },
    { id: "linker", label: "Linkers" },
    { id: "signal_peptide", label: "Signals" },
    { id: "regulatory", label: "Regulatory" },
  ];

  async function loadParts() {
    partsLoading.value = true;
    try {
      if (partsFilter.value.trim()) {
        partsList.value = await partsApi.search(partsFilter.value);
      } else {
        partsList.value = await partsApi.list(partsTypeFilter.value || undefined);
      }
    } catch {
      // Backend not available
    }
    partsLoading.value = false;
  }

  $effect(() => {
    partsFilter.value;
    partsTypeFilter.value;
    loadParts();
  });

  function handleFilterType(type: string) {
    partsTypeFilter.value = type;
    partsFilter.value = "";
  }
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-base-700 px-3 py-2">
    <h2 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider">Parts Library</h2>
    <input
      bind:value={partsFilter.value}
      placeholder="Search parts..."
      class="w-full rounded-md border border-base-600 bg-base-700 px-2.5 py-1.5 text-xs text-base-100 outline-none focus:border-accent placeholder:text-base-500"
    />
  </div>

  <div class="flex flex-wrap gap-1 border-b border-base-700 px-3 py-2">
    {#each partTypes as pt}
      <button
        onclick={() => handleFilterType(pt.id)}
        class="rounded-full px-2 py-0.5 text-[10px] transition-colors
          {partsTypeFilter.value === pt.id
            ? 'bg-accent/20 text-accent'
            : 'bg-base-700 text-base-400 hover:bg-base-600 hover:text-base-200'}"
      >
        {pt.label}
      </button>
    {/each}
  </div>

  <div class="flex-1 overflow-y-auto p-2 space-y-1">
    {#if partsLoading.value}
      <p class="px-2 py-4 text-center text-xs text-base-400">Loading...</p>
    {:else if partsList.value.length === 0}
      <p class="px-2 py-4 text-center text-xs text-base-400">No parts found</p>
    {:else}
      {#each partsList.value as part}
        <PartCard {part} />
      {/each}
    {/if}
  </div>
</div>
