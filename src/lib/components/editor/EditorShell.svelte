<script lang="ts">
  import SchematicView from "./SchematicView.svelte";
  import SequenceView from "./SequenceView.svelte";
  import CircularMap from "./CircularMap.svelte";
  import { activeEditorView, rightSidebarOpen } from "../../stores/ui.svelte";
  import { currentConstruct } from "../../stores/construct.svelte";

  const views = [
    { id: "schematic", label: "Schematic" },
    { id: "sequence", label: "Sequence" },
    { id: "circular", label: "Circular" },
    { id: "split", label: "Split" },
  ] as const;
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-base-700 bg-base-800 px-3 py-1.5">
    <div class="flex gap-1">
      {#each views as v}
        <button
          onclick={() => activeEditorView.value = v.id}
          class="rounded px-3 py-1 text-xs font-medium transition-colors
            {activeEditorView.value === v.id
              ? 'bg-accent/20 text-accent'
              : 'text-base-400 hover:bg-base-700 hover:text-base-200'}"
        >
          {v.label}
        </button>
      {/each}
    </div>
    <div class="flex items-center gap-2">
      {#if currentConstruct.value}
        <span class="text-xs text-base-400">
          {currentConstruct.value.sequence.length.toLocaleString()} bp
        </span>
      {/if}
      <button
        onclick={() => rightSidebarOpen.value = !rightSidebarOpen.value}
        class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200"
        title="Toggle sidebar"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/><line x1="15" y1="3" x2="15" y2="21"/>
        </svg>
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-hidden bg-base-900">
    {#if !currentConstruct.value}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" class="mx-auto mb-4 text-base-500">
            <circle cx="12" cy="12" r="10"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/><line x1="2" y1="12" x2="22" y2="12"/>
          </svg>
          <p class="text-base-400 mb-2">No construct loaded</p>
          <p class="text-xs text-base-500">Create a new construct or open one from your repos</p>
        </div>
      </div>
    {:else if activeEditorView.value === "schematic"}
      <SchematicView />
    {:else if activeEditorView.value === "sequence"}
      <SequenceView />
    {:else if activeEditorView.value === "circular"}
      <CircularMap />
    {:else if activeEditorView.value === "split"}
      <div class="grid h-full grid-cols-2 grid-rows-2 gap-px bg-base-700">
        <div class="bg-base-900"><SchematicView /></div>
        <div class="bg-base-900 row-span-2"><CircularMap /></div>
        <div class="bg-base-900"><SequenceView /></div>
      </div>
    {/if}
  </div>
</div>
