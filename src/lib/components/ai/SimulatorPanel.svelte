<script lang="ts">
  import { currentConstruct, constructParts } from "../../stores/construct.svelte";
  import { ai } from "../../api/tauri";
  import type { SimulatorOutput } from "../../api/tauri";
  import Button from "../common/Button.svelte";

  let host = $state("CHO");
  let copyNumber = $state(10);
  let timeHours = $state(48);
  let loading = $state(false);
  let result = $state<SimulatorOutput | null>(null);
  let error = $state("");

  const hosts = ["CHO", "HEK293", "E. coli", "Yeast"];

  async function runSimulation() {
    if (!currentConstruct.value) {
      error = "No construct loaded";
      return;
    }

    loading = true;
    error = "";
    result = null;

    try {
      result = await ai.simulate({
        construct_name: currentConstruct.value.name,
        sequence: currentConstruct.value.sequence,
        parts_summary: constructParts.value.map((p) => `${p.name} (${p.part_type})`),
        host,
        copy_number: copyNumber,
        time_hours: timeHours,
      });
    } catch (e) {
      error = String(e);
    }

    loading = false;
  }
</script>

<div class="p-3 space-y-4">
  <h3 class="text-xs font-semibold text-base-300 uppercase tracking-wider">Simulator</h3>

  {#if !currentConstruct.value}
    <p class="text-xs text-base-400">Load a construct to run simulations.</p>
  {:else}
    <div class="space-y-3">
      <div class="rounded-lg bg-base-800 p-3">
        <p class="text-xs text-base-300 font-medium">{currentConstruct.value.name}</p>
        <p class="text-xs text-base-400">{currentConstruct.value.sequence.length.toLocaleString()} bp · {constructParts.value.length} parts</p>
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-base-300">Host</label>
        <select bind:value={host} class="w-full rounded border border-base-600 bg-base-700 px-3 py-2 text-xs text-base-100 outline-none focus:border-accent">
          {#each hosts as h}
            <option value={h}>{h}</option>
          {/each}
        </select>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="mb-1 block text-xs font-medium text-base-300">Copy #</label>
          <input type="number" bind:value={copyNumber} min={1} max={1000}
            class="w-full rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent" />
        </div>
        <div>
          <label class="mb-1 block text-xs font-medium text-base-300">Time (h)</label>
          <input type="number" bind:value={timeHours} min={1} max={168}
            class="w-full rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent" />
        </div>
      </div>

      {#if error}
        <p class="text-xs text-red-400">{error}</p>
      {/if}

      <Button variant="primary" onclick={runSimulation} disabled={loading}>
        {loading ? "Simulating..." : "Run Simulation"}
      </Button>

      {#if result}
        <div class="space-y-3">
          <div class="grid grid-cols-2 gap-2">
            <div class="rounded-lg bg-base-800 p-3 text-center">
              <p class="text-lg font-bold text-accent">{result.mrna_level.toFixed(1)}</p>
              <p class="text-[10px] text-base-400">mRNA level (rel.)</p>
            </div>
            <div class="rounded-lg bg-base-800 p-3 text-center">
              <p class="text-lg font-bold text-accent">{result.protein_level.toFixed(1)}</p>
              <p class="text-[10px] text-base-400">Protein level (rel.)</p>
            </div>
          </div>

          <div class="rounded-lg bg-base-800 p-3">
            <div class="flex justify-between mb-1">
              <span class="text-xs text-base-300">Developability</span>
              <span class="text-xs font-bold text-accent">{result.developability_score.toFixed(0)}/100</span>
            </div>
            <div class="h-2 rounded-full bg-base-700">
              <div
                class="h-2 rounded-full bg-accent transition-all"
                style="width: {result.developability_score}%"
              ></div>
            </div>
          </div>

          {#if result.bottlenecks.length > 0}
            <div>
              <p class="text-xs font-medium text-base-300 mb-1">Bottlenecks</p>
              {#each result.bottlenecks as b}
                <div class="rounded bg-yellow-500/10 border border-yellow-500/20 px-2 py-1 text-xs text-yellow-400 mb-1">{b}</div>
              {/each}
            </div>
          {/if}

          {#if result.notes}
            <p class="text-xs text-base-400">{result.notes}</p>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>
