<script lang="ts">
  import { currentConstruct, selection } from "../../stores/construct.svelte";
  import { sequence as seqApi } from "../../api/tauri";
  import type { CutSite, GCResult, TmResult, ORF } from "../../api/tauri";

  let restrictionSites = $state<CutSite[]>([]);
  let uniqueSites = $derived(restrictionSites.filter((s) => s.is_unique));
  let gcResult = $state<GCResult | null>(null);
  let tmResult = $state<TmResult | null>(null);
  let orfs = $state<ORF[]>([]);
  let searchQuery = $state("");
  let searchResults = $state<[number, number][]>([]);
  let useRegex = $state(false);
  let showAllEnzymes = $state(false);

  async function analyzeSequence() {
    if (!currentConstruct.value?.sequence) return;
    const seq = currentConstruct.value.sequence;
    try {
      const [sites, gc, foundOrfs] = await Promise.all([
        seqApi.restrictionSites(seq),
        seqApi.gcContent(seq, 100),
        seqApi.findOrfs(seq, 30),
      ]);
      restrictionSites = sites;
      gcResult = gc;
      orfs = foundOrfs;
    } catch {}
  }

  async function computeTm() {
    if (
      !currentConstruct.value?.sequence ||
      selection.value.type === "none" ||
      selection.value.start === undefined ||
      selection.value.end === undefined
    )
      return;
    const subseq = currentConstruct.value.sequence.slice(selection.value.start, selection.value.end);
    if (subseq.length < 5 || subseq.length > 60) return;
    try {
      tmResult = await seqApi.meltingTemp(subseq);
    } catch {}
  }

  async function runSearch() {
    if (!currentConstruct.value?.sequence || !searchQuery.trim()) {
      searchResults = [];
      return;
    }
    try {
      searchResults = await seqApi.find(currentConstruct.value.sequence, searchQuery, useRegex);
    } catch {
      searchResults = [];
    }
  }

  $effect(() => {
    if (currentConstruct.value) analyzeSequence();
  });

  $effect(() => {
    selection.value;
    computeTm();
  });

  $effect(() => {
    searchQuery;
    useRegex;
    runSearch();
  });
</script>

<div class="p-3 space-y-4">
  <!-- Sequence Find -->
  <section>
    <h3 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider">Find in Sequence</h3>
    <div class="flex gap-2">
      <input
        bind:value={searchQuery}
        placeholder="ATGC or regex..."
        class="flex-1 rounded-md border border-base-600 bg-base-700 px-2.5 py-1.5 text-xs text-base-100 outline-none focus:border-accent font-mono"
      />
      <label class="flex items-center gap-1 text-xs text-base-400">
        <input type="checkbox" bind:checked={useRegex} class="rounded" />
        Regex
      </label>
    </div>
    {#if searchResults.length > 0}
      <p class="mt-1 text-xs text-base-400">{searchResults.length} match{searchResults.length === 1 ? "" : "es"}</p>
    {/if}
  </section>

  <!-- GC Content -->
  {#if gcResult}
    <section>
      <h3 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider">GC Content</h3>
      <div class="rounded-lg bg-base-800 p-3">
        <div class="text-2xl font-bold text-accent">{(gcResult.overall * 100).toFixed(1)}%</div>
        {#if gcResult.window_data.length > 0}
          <div class="mt-2 flex h-8 items-end gap-px">
            {#each gcResult.window_data.filter((_, i) => i % Math.max(1, Math.floor(gcResult!.window_data.length / 100)) === 0) as val}
              <div
                class="flex-1 rounded-t bg-accent/60"
                style="height: {Math.max(2, val * 100)}%"
              ></div>
            {/each}
          </div>
        {/if}
      </div>
    </section>
  {/if}

  <!-- Melting Temperature -->
  {#if tmResult}
    <section>
      <h3 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider">Melting Temp</h3>
      <div class="rounded-lg bg-base-800 p-3 space-y-1">
        <div class="flex justify-between text-xs">
          <span class="text-base-400">Basic Tm</span>
          <span class="text-base-100 font-mono">{tmResult.tm_basic.toFixed(1)} °C</span>
        </div>
        <div class="flex justify-between text-xs">
          <span class="text-base-400">NN Tm</span>
          <span class="text-base-100 font-mono">{tmResult.tm_nearest_neighbor.toFixed(1)} °C</span>
        </div>
        <div class="flex justify-between text-xs">
          <span class="text-base-400">Length</span>
          <span class="text-base-100 font-mono">{tmResult.length} nt</span>
        </div>
      </div>
    </section>
  {/if}

  <!-- Restriction Sites -->
  <section>
    <h3 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider flex items-center justify-between">
      Restriction Sites
      <span class="text-[10px] font-normal text-base-500">{uniqueSites.length} unique</span>
    </h3>
    <div class="rounded-lg bg-base-800 max-h-48 overflow-y-auto">
      {#if uniqueSites.length === 0 && restrictionSites.length === 0}
        <p class="p-3 text-xs text-base-500">No sites found</p>
      {:else}
        {#each (showAllEnzymes ? restrictionSites : uniqueSites).slice(0, 50) as site}
          <div class="flex justify-between px-3 py-1 text-xs border-b border-base-700 last:border-0">
            <span class="font-mono text-base-100">{site.enzyme}</span>
            <span class="text-base-400">
              {site.position + 1}
              {#if site.is_unique}
                <span class="text-green-400 ml-1">unique</span>
              {/if}
            </span>
          </div>
        {/each}
      {/if}
    </div>
    <button
      onclick={() => showAllEnzymes = !showAllEnzymes}
      class="mt-1 text-[10px] text-accent hover:text-accent-hover"
    >
      {showAllEnzymes ? "Show unique only" : "Show all cutters"}
    </button>
  </section>

  <!-- ORFs -->
  <section>
    <h3 class="mb-2 text-xs font-semibold text-base-300 uppercase tracking-wider flex items-center justify-between">
      Open Reading Frames
      <span class="text-[10px] font-normal text-base-500">{orfs.length} found</span>
    </h3>
    <div class="rounded-lg bg-base-800 max-h-48 overflow-y-auto">
      {#each orfs.slice(0, 20) as orf}
        <div class="flex justify-between px-3 py-1 text-xs border-b border-base-700 last:border-0">
          <span class="font-mono text-base-100">
            {orf.start + 1}..{orf.stop}
          </span>
          <span class="text-base-400">
            {orf.length_aa} aa · Frame {orf.frame} · {orf.strand}
          </span>
        </div>
      {/each}
      {#if orfs.length === 0}
        <p class="p-3 text-xs text-base-500">No ORFs found (min 30 aa)</p>
      {/if}
    </div>
  </section>
</div>
