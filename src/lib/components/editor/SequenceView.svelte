<script lang="ts">
  import { onMount } from "svelte";
  import { SequenceApp } from "../../pixi/sequence/SequenceApp";
  import { currentConstruct, constructParts, selection, selectRange } from "../../stores/construct.svelte";

  let canvasEl: HTMLCanvasElement;
  let seqApp: SequenceApp;

  onMount(() => {
    seqApp = new SequenceApp();
    seqApp.init(canvasEl).then(() => {
      seqApp.setCallbacks({
        onSelect: (start, end) => selectRange(start, end),
      });
      renderSequence();
    });

    const observer = new ResizeObserver(() => seqApp?.resize());
    observer.observe(canvasEl.parentElement!);

    return () => {
      observer.disconnect();
      seqApp?.destroy();
    };
  });

  function renderSequence() {
    if (!seqApp || !currentConstruct.value) return;
    const range =
      selection.value.type !== "none" && selection.value.start !== undefined && selection.value.end !== undefined
        ? { start: selection.value.start, end: selection.value.end }
        : undefined;
    seqApp.render(currentConstruct.value.sequence, constructParts.value, range);
  }

  $effect(() => {
    currentConstruct.value;
    constructParts.value;
    selection.value;
    renderSequence();
  });

  $effect(() => {
    if (seqApp && selection.value.type !== "none" && selection.value.start !== undefined && selection.value.end !== undefined) {
      seqApp.highlight(selection.value.start, selection.value.end);
    }
  });
</script>

<div class="h-full w-full relative" role="application" aria-label="Sequence view">
  <canvas bind:this={canvasEl} class="h-full w-full"></canvas>
</div>
