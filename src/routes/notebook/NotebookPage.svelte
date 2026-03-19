<script lang="ts">
  import { notebooks as notebooksApi, repos as reposApi } from "../../lib/api/tauri";
  import type { Notebook, Repo } from "../../lib/api/tauri";
  import Button from "../../lib/components/common/Button.svelte";
  import Modal from "../../lib/components/common/Modal.svelte";
  import NotebookEditor from "../../lib/components/notebook/NotebookEditor.svelte";

  let reposList = $state<Repo[]>([]);
  let selectedRepo = $state<Repo | null>(null);
  let notebooksList = $state<Notebook[]>([]);
  let currentNotebook = $state<Notebook | null>(null);
  let showNewNotebook = $state(false);
  let newTitle = $state("Untitled Notebook");

  async function loadRepos() {
    try {
      reposList = await reposApi.list();
    } catch {}
  }

  async function selectRepo(repo: Repo) {
    selectedRepo = repo;
    currentNotebook = null;
    try {
      notebooksList = await notebooksApi.list(repo.id);
    } catch {}
  }

  async function createNotebook() {
    if (!selectedRepo || !newTitle.trim()) return;
    try {
      const nb = await notebooksApi.create(selectedRepo.id, newTitle);
      notebooksList = [nb, ...notebooksList];
      currentNotebook = nb;
      showNewNotebook = false;
      newTitle = "Untitled Notebook";
    } catch {}
  }

  async function openNotebook(nb: Notebook) {
    try {
      currentNotebook = await notebooksApi.get(nb.id);
    } catch {}
  }

  async function saveNotebook(title: string, content: string) {
    if (!currentNotebook) return;
    try {
      currentNotebook = await notebooksApi.update(currentNotebook.id, title, content);
      notebooksList = notebooksList.map((n) =>
        n.id === currentNotebook!.id ? currentNotebook! : n,
      );
    } catch {}
  }

  async function deleteNotebook(id: string) {
    try {
      await notebooksApi.delete(id);
      notebooksList = notebooksList.filter((n) => n.id !== id);
      if (currentNotebook?.id === id) currentNotebook = null;
    } catch {}
  }

  $effect(() => {
    loadRepos();
  });
</script>

<div class="flex h-full">
  <div class="w-64 flex-shrink-0 border-r border-base-700 flex flex-col">
    <div class="border-b border-base-700 px-4 py-3">
      <h2 class="text-sm font-semibold text-base-200 mb-2">Notebook</h2>
      <p class="text-[10px] text-base-400">Lab notebook for documenting designs, simulations, and observations.</p>
    </div>

    <div class="border-b border-base-700 px-3 py-2">
      <p class="text-[10px] font-medium text-base-400 uppercase mb-1">Select Repo</p>
      <div class="space-y-0.5 max-h-32 overflow-y-auto">
        {#each reposList as repo}
          <button
            onclick={() => selectRepo(repo)}
            class="flex w-full items-center gap-1.5 rounded px-2 py-1 text-left text-[11px] transition-colors
              {selectedRepo?.id === repo.id ? 'bg-accent/20 text-accent' : 'text-base-300 hover:bg-base-700'}"
          >
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/></svg>
            <span class="truncate">{repo.name}</span>
          </button>
        {/each}
      </div>
    </div>

    {#if selectedRepo}
      <div class="flex items-center justify-between border-b border-base-700 px-3 py-2">
        <span class="text-[10px] font-medium text-base-400 uppercase">Entries</span>
        <button
          onclick={() => showNewNotebook = true}
          class="rounded bg-accent/20 px-1.5 py-0.5 text-[10px] text-accent hover:bg-accent/30"
        >
          + New
        </button>
      </div>
      <div class="flex-1 overflow-y-auto p-2 space-y-0.5">
        {#each notebooksList as nb}
          <div class="group flex items-center gap-1">
            <button
              onclick={() => openNotebook(nb)}
              class="flex-1 rounded px-2 py-1.5 text-left text-[11px] transition-colors
                {currentNotebook?.id === nb.id ? 'bg-accent/20 text-accent' : 'text-base-300 hover:bg-base-700'}"
            >
              <span class="block truncate font-medium">{nb.title}</span>
              <span class="block text-[9px] text-base-500">{new Date(nb.updated_at).toLocaleDateString()}</span>
            </button>
            <button
              onclick={() => deleteNotebook(nb.id)}
              class="rounded p-0.5 text-base-500 opacity-0 group-hover:opacity-100 hover:bg-red-500/20 hover:text-red-400 transition-all"
              title="Delete"
            >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            </button>
          </div>
        {/each}
        {#if notebooksList.length === 0}
          <p class="text-center text-[10px] text-base-400 py-4">No entries yet</p>
        {/if}
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center p-4">
        <p class="text-xs text-base-400 text-center">Select a repo to view or create notebook entries</p>
      </div>
    {/if}
  </div>

  <div class="flex-1 overflow-hidden">
    {#if currentNotebook}
      <NotebookEditor
        notebook={currentNotebook}
        onsave={saveNotebook}
      />
    {:else}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" class="mx-auto mb-4 text-base-500">
            <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
          </svg>
          <p class="text-base-400 mb-2">Lab Notebook</p>
          <p class="text-xs text-base-500">Select a repo and open or create a notebook entry</p>
        </div>
      </div>
    {/if}
  </div>
</div>

<Modal open={showNewNotebook} title="New Notebook Entry" onclose={() => showNewNotebook = false}>
  <div class="space-y-4">
    <div>
      <label for="nb-title" class="mb-1 block text-xs font-medium text-base-300">Title</label>
      <input
        id="nb-title"
        bind:value={newTitle}
        class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent"
        placeholder="Lab Entry #1"
      />
    </div>
    <div class="flex justify-end gap-2">
      <Button onclick={() => showNewNotebook = false}>Cancel</Button>
      <Button variant="primary" onclick={createNotebook}>Create</Button>
    </div>
  </div>
</Modal>
