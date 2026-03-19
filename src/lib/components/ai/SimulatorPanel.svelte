<script lang="ts">
  import { currentConstruct, constructParts } from "../../stores/construct.svelte";
  import { ai, odeSimulator } from "../../api/tauri";
  import type { SimulatorOutput, OdeSimResult, CircuitPart } from "../../api/tauri";
  import Button from "../common/Button.svelte";
  import SimulatorGraph from "./SimulatorGraph.svelte";

  let mode = $state<"ode" | "ai">("ode");
  let host = $state("E. coli");
  let copyNumber = $state(10);
  let timeHours = $state(20);
  let loading = $state(false);
  let odeResult = $state<OdeSimResult | null>(null);
  let aiResult = $state<SimulatorOutput | null>(null);
  let error = $state("");
  let detectedCircuit = $state("");
  let showInfo = $state(false);

  let odeAlpha = $state(216);
  let odeAlpha0 = $state(0.216);
  let odeBeta = $state(5);
  let odeN = $state(2);
  let odeKm = $state(40);

  const hosts = ["E. coli", "Yeast", "CHO", "HEK293"];

  $effect(() => {
    if (constructParts.value.length > 0) {
      detectCircuitType();
    }
  });

  async function detectCircuitType() {
    try {
      const parts: CircuitPart[] = constructParts.value.map((p) => ({
        name: p.name,
        part_type: p.part_type,
      }));
      detectedCircuit = await odeSimulator.detectCircuit(parts);
    } catch {
      detectedCircuit = "simple_expression";
    }
  }

  async function runOdeSimulation() {
    if (!currentConstruct.value) {
      error = "No construct loaded";
      return;
    }

    loading = true;
    error = "";
    odeResult = null;

    try {
      const parts: CircuitPart[] = constructParts.value.map((p) => ({
        name: p.name,
        part_type: p.part_type,
      }));
      odeResult = await odeSimulator.run(
        "auto",
        parts,
        timeHours,
        0.005,
        { alpha: odeAlpha, alpha0: odeAlpha0, beta: odeBeta, n: odeN, k_m: odeKm },
      );
      detectedCircuit = odeResult.circuit_type;
    } catch (e) {
      error = String(e);
    }

    loading = false;
  }

  async function runAiSimulation() {
    if (!currentConstruct.value) {
      error = "No construct loaded";
      return;
    }

    loading = true;
    error = "";
    aiResult = null;

    try {
      aiResult = await ai.simulate({
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

  const circuitLabels: Record<string, string> = {
    repressilator: "Repressilator (3-gene oscillator)",
    toggle_switch: "Toggle Switch (bistable)",
    inverter: "NOT Gate / Inverter",
    simple_expression: "Simple Expression",
    multi_gene: "Multi-gene Circuit",
  };
</script>

<div class="p-3 space-y-3 overflow-y-auto h-full">
  <div class="flex items-center justify-between">
    <h3 class="text-xs font-semibold text-base-300 uppercase tracking-wider">Simulator</h3>
    <button
      onclick={() => showInfo = !showInfo}
      class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-accent"
      title="Help / Info"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>
      </svg>
    </button>
  </div>

  {#if showInfo}
    <div class="rounded-lg bg-accent/10 border border-accent/20 p-3 space-y-2">
      <p class="text-xs font-semibold text-accent">ODE Simulator Guide</p>
      <div class="text-[11px] text-base-300 space-y-1.5">
        <p><strong>What it does:</strong> Deterministic simulation of gene circuit dynamics using ordinary differential equations (ODEs) with Euler integration.</p>
        <p><strong>Supported circuits:</strong></p>
        <ul class="list-disc list-inside ml-1 space-y-0.5">
          <li><strong>Repressilator</strong> — 3-gene ring oscillator (LacI ⊣ TetR ⊣ cI ⊣ LacI). Produces oscillating protein levels.</li>
          <li><strong>Toggle Switch</strong> — Mutual repression (LacI ⊣⊢ TetR). Bistable, settles to one state.</li>
          <li><strong>Inverter (NOT gate)</strong> — Single repressor inverts input signal.</li>
          <li><strong>Simple Expression</strong> — Constitutive transcription/translation to steady state.</li>
        </ul>
        <p><strong>Parameters:</strong></p>
        <ul class="list-disc list-inside ml-1 space-y-0.5">
          <li><strong>α (alpha)</strong> — Max transcription rate in absence of repressor</li>
          <li><strong>α₀ (alpha0)</strong> — Basal (leaky) transcription rate</li>
          <li><strong>β (beta)</strong> — Protein/mRNA ratio (translation efficiency)</li>
          <li><strong>n</strong> — Hill coefficient (cooperativity of repression)</li>
          <li><strong>Km</strong> — Half-maximal repression concentration</li>
        </ul>
        <p><strong>How to use:</strong></p>
        <ol class="list-decimal list-inside ml-1 space-y-0.5">
          <li>Load a construct from the Bacterial Demos repo (or build your own)</li>
          <li>Add parts (e.g. pLac→LacI, pTet→TetR, pLambda→cI for Repressilator)</li>
          <li>The simulator auto-detects the circuit type from your parts</li>
          <li>Adjust parameters and duration, then click "Run ODE Simulation"</li>
          <li>View time-course graphs showing protein concentrations over time</li>
          <li>For Repressilator: expect oscillatory waves; for Toggle Switch: bistable steady state</li>
        </ol>
        <p><strong>Export:</strong> Click "Copy Graph as Image" to save the graph for your Notebook.</p>
      </div>
      <button onclick={() => showInfo = false} class="text-[10px] text-accent hover:underline">Close</button>
    </div>
  {/if}

  {#if !currentConstruct.value}
    <p class="text-xs text-base-400">Load a construct to run simulations.</p>
  {:else}
    <div class="space-y-3">
      <div class="rounded-lg bg-base-800 p-3">
        <p class="text-xs text-base-300 font-medium">{currentConstruct.value.name}</p>
        <p class="text-xs text-base-400">{currentConstruct.value.sequence.length.toLocaleString()} bp · {constructParts.value.length} parts</p>
        {#if detectedCircuit}
          <p class="text-xs text-accent mt-1">
            Detected: {circuitLabels[detectedCircuit] ?? detectedCircuit}
          </p>
        {/if}
      </div>

      <div class="flex gap-1">
        <button
          onclick={() => mode = "ode"}
          class="flex-1 rounded px-2 py-1 text-xs font-medium transition-colors
            {mode === 'ode' ? 'bg-accent/20 text-accent' : 'text-base-400 hover:bg-base-700'}"
        >
          ODE Sim
        </button>
        <button
          onclick={() => mode = "ai"}
          class="flex-1 rounded px-2 py-1 text-xs font-medium transition-colors
            {mode === 'ai' ? 'bg-accent/20 text-accent' : 'text-base-400 hover:bg-base-700'}"
        >
          AI Sim
        </button>
      </div>

      {#if mode === "ode"}
        <div>
          <label for="sim-duration" class="mb-1 block text-xs font-medium text-base-300">Duration (hours)</label>
          <input id="sim-duration" type="number" bind:value={timeHours} min={1} max={200}
            class="w-full rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent" />
        </div>

        <details class="group">
          <summary class="cursor-pointer text-xs font-medium text-base-400 hover:text-base-200">
            Advanced Parameters ▸
          </summary>
          <div class="mt-2 grid grid-cols-2 gap-2">
            <div>
              <label for="sim-alpha" class="mb-0.5 block text-[10px] text-base-400">α (max txn)</label>
              <input id="sim-alpha" type="number" bind:value={odeAlpha} step={1}
                class="w-full rounded border border-base-600 bg-base-700 px-1.5 py-1 text-[11px] text-base-100 outline-none focus:border-accent" />
            </div>
            <div>
              <label for="sim-alpha0" class="mb-0.5 block text-[10px] text-base-400">α₀ (basal)</label>
              <input id="sim-alpha0" type="number" bind:value={odeAlpha0} step={0.01}
                class="w-full rounded border border-base-600 bg-base-700 px-1.5 py-1 text-[11px] text-base-100 outline-none focus:border-accent" />
            </div>
            <div>
              <label for="sim-beta" class="mb-0.5 block text-[10px] text-base-400">β (translation)</label>
              <input id="sim-beta" type="number" bind:value={odeBeta} step={0.5}
                class="w-full rounded border border-base-600 bg-base-700 px-1.5 py-1 text-[11px] text-base-100 outline-none focus:border-accent" />
            </div>
            <div>
              <label for="sim-n" class="mb-0.5 block text-[10px] text-base-400">n (Hill coeff)</label>
              <input id="sim-n" type="number" bind:value={odeN} step={0.1}
                class="w-full rounded border border-base-600 bg-base-700 px-1.5 py-1 text-[11px] text-base-100 outline-none focus:border-accent" />
            </div>
            <div class="col-span-2">
              <label for="sim-km" class="mb-0.5 block text-[10px] text-base-400">Km (half-max)</label>
              <input id="sim-km" type="number" bind:value={odeKm} step={1}
                class="w-full rounded border border-base-600 bg-base-700 px-1.5 py-1 text-[11px] text-base-100 outline-none focus:border-accent" />
            </div>
          </div>
        </details>

        {#if error}
          <p class="text-xs text-red-400">{error}</p>
        {/if}

        <Button variant="primary" onclick={runOdeSimulation} disabled={loading}>
          {loading ? "Simulating..." : "Run ODE Simulation"}
        </Button>

        {#if odeResult}
          <SimulatorGraph result={odeResult} />

          {#if odeResult.period_hours}
            <div class="rounded-lg bg-base-800 p-3">
              <p class="text-xs text-base-300">Oscillation period: <span class="font-bold text-accent">{odeResult.period_hours.toFixed(1)} h</span></p>
            </div>
          {/if}

          <div class="rounded-lg bg-base-800 p-3">
            <p class="text-[11px] text-base-400 whitespace-pre-wrap">{odeResult.notes}</p>
          </div>
        {/if}
      {:else}
        <div>
          <label class="mb-1 block text-xs font-medium text-base-300">
            Host
            <select bind:value={host} class="mt-1 block w-full rounded border border-base-600 bg-base-700 px-3 py-2 text-xs text-base-100 outline-none focus:border-accent">
              {#each hosts as h}
                <option value={h}>{h}</option>
              {/each}
            </select>
          </label>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="mb-1 block text-xs font-medium text-base-300">
              Copy #
              <input type="number" bind:value={copyNumber} min={1} max={1000}
                class="mt-1 block w-full rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent" />
            </label>
          </div>
          <div>
            <label class="mb-1 block text-xs font-medium text-base-300">
              Time (h)
              <input type="number" bind:value={timeHours} min={1} max={168}
                class="mt-1 block w-full rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent" />
            </label>
          </div>
        </div>

        {#if error}
          <p class="text-xs text-red-400">{error}</p>
        {/if}

        <Button variant="primary" onclick={runAiSimulation} disabled={loading}>
          {loading ? "Simulating..." : "Run AI Simulation"}
        </Button>

        {#if aiResult}
          <div class="space-y-3">
            <div class="grid grid-cols-2 gap-2">
              <div class="rounded-lg bg-base-800 p-3 text-center">
                <p class="text-lg font-bold text-accent">{aiResult.mrna_level.toFixed(1)}</p>
                <p class="text-[10px] text-base-400">mRNA level (rel.)</p>
              </div>
              <div class="rounded-lg bg-base-800 p-3 text-center">
                <p class="text-lg font-bold text-accent">{aiResult.protein_level.toFixed(1)}</p>
                <p class="text-[10px] text-base-400">Protein level (rel.)</p>
              </div>
            </div>

            <div class="rounded-lg bg-base-800 p-3">
              <div class="flex justify-between mb-1">
                <span class="text-xs text-base-300">Developability</span>
                <span class="text-xs font-bold text-accent">{aiResult.developability_score.toFixed(0)}/100</span>
              </div>
              <div class="h-2 rounded-full bg-base-700">
                <div
                  class="h-2 rounded-full bg-accent transition-all"
                  style="width: {aiResult.developability_score}%"
                ></div>
              </div>
            </div>

            {#if aiResult.bottlenecks.length > 0}
              <div>
                <p class="text-xs font-medium text-base-300 mb-1">Bottlenecks</p>
                {#each aiResult.bottlenecks as b}
                  <div class="rounded bg-yellow-500/10 border border-yellow-500/20 px-2 py-1 text-xs text-yellow-400 mb-1">{b}</div>
                {/each}
              </div>
            {/if}

            {#if aiResult.notes}
              <p class="text-xs text-base-400">{aiResult.notes}</p>
            {/if}
          </div>
        {/if}
      {/if}
    </div>
  {/if}
</div>
