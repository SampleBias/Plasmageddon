<script lang="ts">
  import type { Part } from "../../stores/parts.svelte";

  interface Props {
    part: Part;
  }

  let { part }: Props = $props();

  const typeColors: Record<string, string> = {
    promoter: "bg-promoter/20 text-promoter border-promoter/30",
    cds: "bg-cds/20 text-cds border-cds/30",
    terminator: "bg-terminator/20 text-terminator border-terminator/30",
    ori: "bg-ori/20 text-ori border-ori/30",
    marker: "bg-marker/20 text-marker border-marker/30",
    tag: "bg-tag/20 text-tag border-tag/30",
    linker: "bg-linker/20 text-linker border-linker/30",
    signal_peptide: "bg-signal/20 text-signal border-signal/30",
    regulatory: "bg-accent/20 text-accent border-accent/30",
    other: "bg-base-600/20 text-base-400 border-base-600/30",
  };

  function handleDragStart(e: DragEvent) {
    if (e.dataTransfer) {
      e.dataTransfer.setData("text/plain", part.id);
      e.dataTransfer.effectAllowed = "copy";
    }
  }
</script>

<div
  role="button"
  tabindex="0"
  draggable="true"
  ondragstart={handleDragStart}
  aria-label="Drag {part.name} to add to construct"
  class="group cursor-grab rounded-lg border border-base-700 bg-base-800 px-3 py-2 transition-colors hover:border-base-500 active:cursor-grabbing"
>
  <div class="flex items-center justify-between">
    <span class="text-xs font-medium text-base-100 truncate">{part.name}</span>
    <span class="rounded-full border px-1.5 py-0.5 text-[9px] {typeColors[part.part_type] ?? typeColors.other}">
      {part.part_type}
    </span>
  </div>
  <div class="mt-1 flex items-center gap-2 text-[10px] text-base-400">
    <span>{part.sequence.length} bp</span>
    {#if part.description}
      <span class="truncate">{part.description}</span>
    {/if}
  </div>
</div>
