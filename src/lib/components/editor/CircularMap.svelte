<script lang="ts">
  import { onMount } from "svelte";
  import { CircularApp } from "../../pixi/circular/CircularApp";
  import { currentConstruct, constructParts, selection, selectPart } from "../../stores/construct.svelte";

  let canvasEl: HTMLCanvasElement;
  let circApp: CircularApp;

  onMount(() => {
    circApp = new CircularApp();
    circApp.init(canvasEl).then(() => {
      circApp.setCallbacks({
        onPartClick: (partId) => selectPart(partId),
      });
      renderCircular();
    });

    const observer = new ResizeObserver(() => {
      circApp?.resize();
      renderCircular();
    });
    observer.observe(canvasEl.parentElement!);

    return () => {
      observer.disconnect();
      circApp?.destroy();
    };
  });

  function renderCircular() {
    if (!circApp || !currentConstruct.value) return;
    const selectedId = selection.value.type === "part" ? selection.value.partId : undefined;
    circApp.render(
      currentConstruct.value.sequence,
      constructParts.value,
      currentConstruct.value.topology,
      selectedId,
    );
  }

  $effect(() => {
    currentConstruct.value;
    constructParts.value;
    selection.value;
    renderCircular();
  });
</script>

<div class="h-full w-full relative" role="application" aria-label="Circular map">
  <canvas bind:this={canvasEl} class="h-full w-full"></canvas>
</div>
