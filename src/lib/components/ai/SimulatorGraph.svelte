<script lang="ts">
  import type { OdeSimResult } from "../../api/tauri";

  interface Props {
    result: OdeSimResult;
  }

  let { result }: Props = $props();
  let canvasEl: HTMLCanvasElement | undefined = $state();
  let copyStatus = $state("");

  $effect(() => {
    if (canvasEl && result) {
      drawGraph();
    }
  });

  function drawGraph() {
    if (!canvasEl || !result) return;

    const dpr = window.devicePixelRatio || 1;
    const w = canvasEl.clientWidth;
    const h = canvasEl.clientHeight;
    canvasEl.width = w * dpr;
    canvasEl.height = h * dpr;

    const ctx = canvasEl.getContext("2d");
    if (!ctx) return;

    ctx.scale(dpr, dpr);
    ctx.clearRect(0, 0, w, h);

    const pad = { top: 20, right: 15, bottom: 40, left: 50 };
    const plotW = w - pad.left - pad.right;
    const plotH = h - pad.top - pad.bottom;

    ctx.fillStyle = "#161822";
    ctx.fillRect(0, 0, w, h);

    const times = result.time_points;
    const maxTime = times[times.length - 1] || 1;
    let maxVal = 0;
    for (const s of result.species) {
      for (const v of s.values) {
        if (v > maxVal) maxVal = v;
      }
    }
    if (maxVal === 0) maxVal = 1;
    maxVal *= 1.1;

    ctx.strokeStyle = "#3a3d52";
    ctx.lineWidth = 0.5;
    const gridLines = 5;
    for (let i = 0; i <= gridLines; i++) {
      const y = pad.top + plotH - (i / gridLines) * plotH;
      ctx.beginPath();
      ctx.moveTo(pad.left, y);
      ctx.lineTo(pad.left + plotW, y);
      ctx.stroke();

      ctx.fillStyle = "#6b7094";
      ctx.font = "10px system-ui";
      ctx.textAlign = "right";
      ctx.fillText(((maxVal * i) / gridLines).toFixed(0), pad.left - 5, y + 3);
    }

    const timeGridLines = 5;
    for (let i = 0; i <= timeGridLines; i++) {
      const x = pad.left + (i / timeGridLines) * plotW;
      ctx.beginPath();
      ctx.moveTo(x, pad.top);
      ctx.lineTo(x, pad.top + plotH);
      ctx.stroke();

      ctx.fillStyle = "#6b7094";
      ctx.font = "10px system-ui";
      ctx.textAlign = "center";
      ctx.fillText(((maxTime * i) / timeGridLines).toFixed(1), x, pad.top + plotH + 15);
    }

    ctx.fillStyle = "#9498b8";
    ctx.font = "11px system-ui";
    ctx.textAlign = "center";
    ctx.fillText("Time (hours)", pad.left + plotW / 2, h - 5);

    ctx.save();
    ctx.translate(12, pad.top + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillStyle = "#9498b8";
    ctx.font = "11px system-ui";
    ctx.textAlign = "center";
    ctx.fillText("Concentration (a.u.)", 0, 0);
    ctx.restore();

    for (const species of result.species) {
      ctx.strokeStyle = species.color;
      ctx.lineWidth = 1.5;
      ctx.beginPath();

      for (let i = 0; i < species.values.length; i++) {
        const x = pad.left + (times[i] / maxTime) * plotW;
        const y = pad.top + plotH - (species.values[i] / maxVal) * plotH;

        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.stroke();
    }

    const legendX = pad.left + 10;
    let legendY = pad.top + 5;
    for (const species of result.species) {
      ctx.fillStyle = species.color;
      ctx.fillRect(legendX, legendY, 12, 3);
      ctx.fillStyle = "#c4c7df";
      ctx.font = "10px system-ui";
      ctx.textAlign = "left";
      ctx.fillText(species.name, legendX + 16, legendY + 4);
      legendY += 14;
    }
  }

  async function copyGraphAsImage() {
    if (!canvasEl) return;
    try {
      const blob = await new Promise<Blob | null>((resolve) =>
        canvasEl!.toBlob(resolve, "image/png"),
      );
      if (blob) {
        await navigator.clipboard.write([
          new ClipboardItem({ "image/png": blob }),
        ]);
        copyStatus = "Copied!";
        setTimeout(() => (copyStatus = ""), 2000);
      }
    } catch {
      copyStatus = "Failed to copy";
      setTimeout(() => (copyStatus = ""), 2000);
    }
  }

  function downloadGraph() {
    if (!canvasEl) return;
    const link = document.createElement("a");
    link.download = `simulation_${result.circuit_type}_${Date.now()}.png`;
    link.href = canvasEl.toDataURL("image/png");
    link.click();
  }
</script>

<div class="space-y-2">
  <canvas
    bind:this={canvasEl}
    class="w-full rounded-lg border border-base-700"
    style="height: 220px"
  ></canvas>
  <div class="flex gap-1">
    <button
      onclick={copyGraphAsImage}
      class="flex-1 rounded bg-base-700 px-2 py-1 text-[10px] text-base-300 hover:bg-base-600 hover:text-base-100 transition-colors"
    >
      {copyStatus || "Copy Graph as Image"}
    </button>
    <button
      onclick={downloadGraph}
      class="rounded bg-base-700 px-2 py-1 text-[10px] text-base-300 hover:bg-base-600 hover:text-base-100 transition-colors"
    >
      Download PNG
    </button>
  </div>
</div>
