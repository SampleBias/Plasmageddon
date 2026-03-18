<script lang="ts">
  import { onMount } from "svelte";
  import { SchematicApp } from "../../pixi/schematic/SchematicApp";
  import { currentConstruct, constructParts, selection, selectPart } from "../../stores/construct.svelte";
  import { constructs as constructsApi, parts as partsApi } from "../../api/tauri";

  let canvasEl: HTMLCanvasElement;
  let schematicApp: SchematicApp;

  onMount(() => {
    schematicApp = new SchematicApp();
    schematicApp.init(canvasEl).then(() => {
      schematicApp.setCallbacks({
        onPartClick: (partId) => selectPart(partId),
        onPartDrop: async (partId, index) => {
          if (!currentConstruct.value) return;
          try {
            const partInfo = await partsApi.get(partId);
            await constructsApi.addPart(
              currentConstruct.value.id,
              partId,
              0,
              1,
              index,
            );
            await refreshParts();
          } catch (e) {
            console.error("Drop failed:", e);
          }
        },
        onReorder: async (partIds) => {
          if (!currentConstruct.value) return;
          try {
            await constructsApi.reorderParts(currentConstruct.value.id, partIds);
            await refreshParts();
          } catch (e) {
            console.error("Reorder failed:", e);
          }
        },
      });
      renderSchematic();
    });

    const observer = new ResizeObserver(() => schematicApp?.resize());
    observer.observe(canvasEl.parentElement!);

    return () => {
      observer.disconnect();
      schematicApp?.destroy();
    };
  });

  async function refreshParts() {
    if (!currentConstruct.value) return;
    try {
      const cps = await constructsApi.getParts(currentConstruct.value.id);
      const withInfo = await Promise.all(
        cps.map(async (cp) => {
          const part = await partsApi.get(cp.part_id);
          return {
            ...cp,
            name: part.name,
            part_type: part.part_type,
            sequence: part.sequence,
            description: part.description,
          };
        }),
      );
      constructParts.value = withInfo;
    } catch {}
  }

  function renderSchematic() {
    if (!schematicApp) return;
    const selectedId = selection.value.type === "part" ? selection.value.partId : undefined;
    schematicApp.render(constructParts.value, selectedId);
  }

  $effect(() => {
    constructParts.value;
    selection.value;
    renderSchematic();
  });

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "copy";
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const partId = e.dataTransfer?.getData("text/plain");
    if (partId && schematicApp) {
      const rect = canvasEl.getBoundingClientRect();
      schematicApp.handleExternalDrop(partId, e.clientX - rect.left);
    }
  }
</script>

<div class="h-full w-full relative" role="application" aria-label="Schematic view">
  <canvas
    bind:this={canvasEl}
    class="h-full w-full"
    ondragover={handleDragOver}
    ondrop={handleDrop}
  ></canvas>
</div>
