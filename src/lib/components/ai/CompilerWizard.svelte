<script lang="ts">
  import { ai } from "../../api/tauri";
  import type { CompilerInput, CompilerOutput } from "../../api/tauri";
  import Button from "../common/Button.svelte";

  let step = $state(1);
  let aaSequences = $state([{ name: "Heavy Chain", sequence: "", chain_type: "VH" }]);
  let architecture = $state("IgG1");
  let host = $state("CHO");
  let loading = $state(false);
  let result = $state<CompilerOutput | null>(null);
  let error = $state("");

  const architectures = ["IgG1", "IgG4", "Bispecific", "scFv", "Fab", "Fusion", "Other"];
  const hosts = ["CHO", "HEK293", "E. coli", "Yeast", "Insect cells"];

  function addChain() {
    aaSequences = [...aaSequences, { name: `Chain ${aaSequences.length + 1}`, sequence: "", chain_type: "VL" }];
  }

  function removeChain(idx: number) {
    aaSequences = aaSequences.filter((_, i) => i !== idx);
  }

  async function compile() {
    const valid = aaSequences.every((aa) => aa.sequence.trim().length > 0);
    if (!valid) {
      error = "All sequences must be filled in";
      return;
    }

    loading = true;
    error = "";
    result = null;

    const input: CompilerInput = {
      aa_sequences: aaSequences.map((aa) => ({
        name: aa.name,
        sequence: aa.sequence.trim().toUpperCase().replace(/\s/g, ""),
        chain_type: aa.chain_type,
      })),
      architecture,
      host,
    };

    try {
      result = await ai.compile(input);
      step = 3;
    } catch (e) {
      error = String(e);
    }

    loading = false;
  }

  function reset() {
    step = 1;
    result = null;
    error = "";
    aaSequences = [{ name: "Heavy Chain", sequence: "", chain_type: "VH" }];
  }
</script>

<div class="p-3 space-y-4">
  <h3 class="text-xs font-semibold text-base-300 uppercase tracking-wider">Compiler</h3>

  {#if step === 1}
    <div class="space-y-3">
      <p class="text-xs text-base-400">Paste amino acid sequences to compile into expression constructs.</p>

      {#each aaSequences as aa, i}
        <div class="rounded-lg border border-base-700 bg-base-800 p-3 space-y-2">
          <div class="flex items-center gap-2">
            <input
              bind:value={aa.name}
              class="flex-1 rounded border border-base-600 bg-base-700 px-2 py-1 text-xs text-base-100 outline-none focus:border-accent"
              placeholder="Chain name"
            />
            <select
              bind:value={aa.chain_type}
              class="rounded border border-base-600 bg-base-700 px-2 py-1 text-xs text-base-100 outline-none"
            >
              <option value="VH">VH</option>
              <option value="VL">VL</option>
              <option value="HC">HC</option>
              <option value="LC">LC</option>
              <option value="scFv">scFv</option>
              <option value="Other">Other</option>
            </select>
            {#if aaSequences.length > 1}
              <button onclick={() => removeChain(i)} class="text-base-400 hover:text-red-400 text-xs">x</button>
            {/if}
          </div>
          <textarea
            bind:value={aa.sequence}
            rows={4}
            class="w-full resize-none rounded border border-base-600 bg-base-700 px-2 py-1.5 text-xs text-base-100 outline-none focus:border-accent font-mono"
            placeholder="MVLQTQVFI..."
          ></textarea>
        </div>
      {/each}

      <button onclick={addChain} class="text-xs text-accent hover:text-accent-hover">+ Add chain</button>

      <div class="flex justify-end">
        <Button variant="primary" onclick={() => step = 2}>Next</Button>
      </div>
    </div>

  {:else if step === 2}
    <div class="space-y-3">
      <div>
        <label class="mb-1 block text-xs font-medium text-base-300">Architecture</label>
        <select bind:value={architecture} class="w-full rounded border border-base-600 bg-base-700 px-3 py-2 text-xs text-base-100 outline-none focus:border-accent">
          {#each architectures as arch}
            <option value={arch}>{arch}</option>
          {/each}
        </select>
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-base-300">Host Organism</label>
        <select bind:value={host} class="w-full rounded border border-base-600 bg-base-700 px-3 py-2 text-xs text-base-100 outline-none focus:border-accent">
          {#each hosts as h}
            <option value={h}>{h}</option>
          {/each}
        </select>
      </div>

      <div class="rounded-lg bg-base-800 p-3">
        <p class="text-xs text-base-300 mb-1">Summary</p>
        <p class="text-xs text-base-400">
          {aaSequences.length} chain{aaSequences.length > 1 ? "s" : ""} · {architecture} · {host}
        </p>
      </div>

      {#if error}
        <p class="text-xs text-red-400">{error}</p>
      {/if}

      <div class="flex justify-between">
        <Button onclick={() => step = 1}>Back</Button>
        <Button variant="primary" onclick={compile} disabled={loading}>
          {loading ? "Compiling..." : "Compile"}
        </Button>
      </div>
    </div>

  {:else if step === 3 && result}
    <div class="space-y-3">
      <div class="rounded-lg bg-green-500/10 border border-green-500/30 p-3">
        <p class="text-xs font-medium text-green-400">Compilation complete</p>
        <p class="text-xs text-green-400/70">{result.constructs.length} construct{result.constructs.length > 1 ? "s" : ""} generated</p>
      </div>

      {#each result.constructs as construct}
        <div class="rounded-lg border border-base-700 bg-base-800 p-3 space-y-2">
          <p class="text-xs font-medium text-base-100">{construct.name}</p>
          <p class="text-xs text-base-400">Signal peptide: {construct.signal_peptide || "None"}</p>
          <p class="text-xs text-base-400">{construct.dna_sequence.length} bp</p>
          <div class="flex flex-wrap gap-1">
            {#each construct.parts as part}
              <span class="rounded bg-base-600 px-1.5 py-0.5 text-[10px] text-base-300">{part.name}</span>
            {/each}
          </div>
        </div>
      {/each}

      {#if result.notes}
        <p class="text-xs text-base-400">{result.notes}</p>
      {/if}

      <Button onclick={reset}>Start Over</Button>
    </div>
  {/if}
</div>
