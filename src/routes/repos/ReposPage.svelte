<script lang="ts">
  import { repos as reposApi, constructs as constructsApi } from "../../lib/api/tauri";
  import type { Repo, Construct } from "../../lib/api/tauri";
  import Button from "../../lib/components/common/Button.svelte";
  import Modal from "../../lib/components/common/Modal.svelte";
  import { navigate } from "../../lib/stores/ui.svelte";
  import { currentConstruct } from "../../lib/stores/construct.svelte";

  let reposList = $state<Repo[]>([]);
  let selectedRepo = $state<Repo | null>(null);
  let repoConstructs = $state<Construct[]>([]);
  let showNewRepo = $state(false);
  let showNewConstruct = $state(false);
  let newRepoName = $state("");
  let newRepoDesc = $state("");
  let newConstructName = $state("");
  let newConstructTopology = $state("circular");

  async function loadRepos() {
    try { reposList = await reposApi.list(); } catch {}
  }

  async function selectRepo(repo: Repo) {
    selectedRepo = repo;
    try { repoConstructs = await constructsApi.list(repo.id); } catch {}
  }

  async function createRepo() {
    if (!newRepoName.trim()) return;
    try {
      const repo = await reposApi.create(newRepoName, newRepoDesc);
      reposList = [repo, ...reposList];
      showNewRepo = false;
      newRepoName = "";
      newRepoDesc = "";
    } catch {}
  }

  async function createConstruct() {
    if (!newConstructName.trim() || !selectedRepo) return;
    try {
      const c = await constructsApi.create(selectedRepo.id, newConstructName, "", newConstructTopology);
      repoConstructs = [c, ...repoConstructs];
      showNewConstruct = false;
      newConstructName = "";
    } catch {}
  }

  async function openConstruct(c: Construct) {
    currentConstruct.value = {
      id: c.id,
      repo_id: c.repo_id,
      name: c.name,
      description: c.description,
      topology: c.topology as "circular" | "linear",
      tags: c.tags,
      sequence: c.sequence,
      created_at: c.created_at,
      updated_at: c.updated_at,
    };
    navigate("/editor");
  }

  async function deleteRepo(id: string) {
    try {
      await reposApi.delete(id);
      reposList = reposList.filter((r) => r.id !== id);
      if (selectedRepo?.id === id) {
        selectedRepo = null;
        repoConstructs = [];
      }
    } catch {}
  }

  $effect(() => { loadRepos(); });
</script>

<div class="flex h-full">
  <div class="w-72 border-r border-base-700 flex flex-col">
    <div class="flex items-center justify-between border-b border-base-700 px-4 py-3">
      <h2 class="text-sm font-semibold text-base-200">Repos</h2>
      <Button size="sm" variant="primary" onclick={() => showNewRepo = true}>+ New</Button>
    </div>
    <div class="flex-1 overflow-y-auto p-2">
      {#each reposList as repo}
        <button
          onclick={() => selectRepo(repo)}
          class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors
            {selectedRepo?.id === repo.id ? 'bg-accent/20 text-accent' : 'text-base-200 hover:bg-base-700'}"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
          </svg>
          <span class="truncate">{repo.name}</span>
        </button>
      {/each}
      {#if reposList.length === 0}
        <p class="px-3 py-6 text-center text-xs text-base-400">No repos yet. Create one to get started.</p>
      {/if}
    </div>
  </div>

  <div class="flex-1 p-6">
    {#if selectedRepo}
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-xl font-bold text-base-100">{selectedRepo.name}</h1>
          <p class="text-sm text-base-400">{selectedRepo.description || "No description"}</p>
        </div>
        <div class="flex gap-2">
          <Button onclick={() => showNewConstruct = true} variant="primary">+ Construct</Button>
          <Button onclick={() => deleteRepo(selectedRepo!.id)} variant="danger">Delete Repo</Button>
        </div>
      </div>

      <div class="space-y-2">
        {#each repoConstructs as c}
          <button
            onclick={() => openConstruct(c)}
            class="flex w-full items-center justify-between rounded-lg border border-base-700 bg-base-800 px-4 py-3 text-left transition-colors hover:border-accent/50"
          >
            <div>
              <span class="text-sm font-medium text-base-100">{c.name}</span>
              <div class="mt-1 flex gap-2">
                <span class="rounded bg-base-600 px-1.5 py-0.5 text-xs text-base-300">{c.topology}</span>
                <span class="text-xs text-base-400">{c.sequence.length.toLocaleString()} bp</span>
              </div>
            </div>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-base-400">
              <polyline points="9 18 15 12 9 6"/>
            </svg>
          </button>
        {/each}
        {#if repoConstructs.length === 0}
          <p class="py-8 text-center text-sm text-base-400">No constructs yet</p>
        {/if}
      </div>
    {:else}
      <div class="flex h-full items-center justify-center text-base-400">
        <p>Select a repo from the sidebar</p>
      </div>
    {/if}
  </div>
</div>

<Modal open={showNewRepo} title="New Repository" onclose={() => showNewRepo = false}>
  <div class="space-y-4">
    <div>
      <label for="repo-name" class="mb-1 block text-xs font-medium text-base-300">Name</label>
      <input id="repo-name" bind:value={newRepoName} class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent" placeholder="My Project" />
    </div>
    <div>
      <label for="repo-desc" class="mb-1 block text-xs font-medium text-base-300">Description</label>
      <input id="repo-desc" bind:value={newRepoDesc} class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent" placeholder="Optional description" />
    </div>
    <div class="flex justify-end gap-2">
      <Button onclick={() => showNewRepo = false}>Cancel</Button>
      <Button variant="primary" onclick={createRepo}>Create</Button>
    </div>
  </div>
</Modal>

<Modal open={showNewConstruct} title="New Construct" onclose={() => showNewConstruct = false}>
  <div class="space-y-4">
    <div>
      <label for="c-name" class="mb-1 block text-xs font-medium text-base-300">Name</label>
      <input id="c-name" bind:value={newConstructName} class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent" placeholder="pCMV-GFP" />
    </div>
    <div>
      <label for="c-topo" class="mb-1 block text-xs font-medium text-base-300">Topology</label>
      <select id="c-topo" bind:value={newConstructTopology} class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent">
        <option value="circular">Circular</option>
        <option value="linear">Linear</option>
      </select>
    </div>
    <div class="flex justify-end gap-2">
      <Button onclick={() => showNewConstruct = false}>Cancel</Button>
      <Button variant="primary" onclick={createConstruct}>Create</Button>
    </div>
  </div>
</Modal>
