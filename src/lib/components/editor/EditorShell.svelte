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

  let exportStatus = $state("");

  async function exportViewAsImage() {
    const viewContainer = document.querySelector('[role="application"]');
    const canvas = viewContainer?.querySelector("canvas");
    if (!canvas) {
      exportStatus = "No canvas to export";
      setTimeout(() => (exportStatus = ""), 2000);
      return;
    }

    try {
      const blob = await new Promise<Blob | null>((resolve) =>
        canvas.toBlob(resolve, "image/png"),
      );
      if (blob) {
        await navigator.clipboard.write([
          new ClipboardItem({ "image/png": blob }),
        ]);
        exportStatus = "Copied to clipboard!";
      }
    } catch {
      try {
        const link = document.createElement("a");
        const name = currentConstruct.value?.name ?? "construct";
        link.download = `${name}_${activeEditorView.value}_${Date.now()}.png`;
        link.href = canvas.toDataURL("image/png");
        link.click();
        exportStatus = "Downloaded!";
      } catch {
        exportStatus = "Export failed";
      }
    }
    setTimeout(() => (exportStatus = ""), 2000);
  }

  function downloadViewAsImage() {
    const viewContainer = document.querySelector('[role="application"]');
    const canvas = viewContainer?.querySelector("canvas");
    if (!canvas) return;
    const link = document.createElement("a");
    const name = currentConstruct.value?.name ?? "construct";
    link.download = `${name}_${activeEditorView.value}_${Date.now()}.png`;
    link.href = canvas.toDataURL("image/png");
    link.click();
  }
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
      {#if exportStatus}
        <span class="text-[10px] text-green-400">{exportStatus}</span>
      {/if}
      {#if currentConstruct.value}
        <button
          onclick={exportViewAsImage}
          class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-accent"
          title="Copy view as image (for Notebook)"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
          </svg>
        </button>
        <button
          onclick={downloadViewAsImage}
          class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-accent"
          title="Download view as PNG"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
        </button>
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
