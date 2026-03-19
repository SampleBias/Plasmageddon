<script lang="ts">
  import type { Notebook } from "../../api/tauri";
  import Button from "../common/Button.svelte";

  interface Props {
    notebook: Notebook;
    onsave: (title: string, content: string) => void;
  }

  let { notebook, onsave }: Props = $props();
  let title = $state("");
  let editorEl: HTMLDivElement | undefined = $state();
  let saving = $state(false);
  let saveStatus = $state("");
  let dirty = $state(false);

  $effect(() => {
    title = notebook.title;
    if (editorEl) {
      editorEl.innerHTML = notebook.content || '<p>Start writing your lab notes here...</p>';
    }
    dirty = false;
  });

  function handleInput() {
    dirty = true;
  }

  function handlePaste(e: ClipboardEvent) {
    const items = e.clipboardData?.items;
    if (!items) return;

    for (const item of items) {
      if (item.type.startsWith("image/")) {
        e.preventDefault();
        const file = item.getAsFile();
        if (!file) continue;

        const reader = new FileReader();
        reader.onload = () => {
          const dataUrl = reader.result as string;
          insertImageAtCursor(dataUrl);
          dirty = true;
        };
        reader.readAsDataURL(file);
        return;
      }
    }
  }

  function insertImageAtCursor(src: string) {
    const sel = window.getSelection();
    if (!sel || !sel.rangeCount) return;

    const range = sel.getRangeAt(0);
    range.deleteContents();

    const container = document.createElement("div");
    container.className = "notebook-image-container";
    container.style.margin = "12px 0";
    container.style.textAlign = "center";

    const img = document.createElement("img");
    img.src = src;
    img.style.maxWidth = "100%";
    img.style.borderRadius = "8px";
    img.style.border = "1px solid #3a3d52";

    container.appendChild(img);
    range.insertNode(container);

    range.setStartAfter(container);
    range.collapse(true);
    sel.removeAllRanges();
    sel.addRange(range);
  }

  async function save() {
    if (!editorEl) return;
    saving = true;
    const content = editorEl.innerHTML;
    onsave(title, content);
    saving = false;
    dirty = false;
    saveStatus = "Saved";
    setTimeout(() => (saveStatus = ""), 2000);
  }

  function execCmd(command: string, value?: string) {
    document.execCommand(command, false, value);
    editorEl?.focus();
    dirty = true;
  }

  function insertHr() {
    execCmd("insertHTML", "<hr style='border-color: #3a3d52; margin: 12px 0;' />");
  }

  function insertTimestamp() {
    const now = new Date().toLocaleString();
    execCmd("insertHTML", `<span style="color: #818cf8; font-size: 0.75rem;">[${now}]</span> `);
  }
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center gap-3 border-b border-base-700 px-4 py-2">
    <input
      bind:value={title}
      oninput={() => dirty = true}
      class="flex-1 bg-transparent text-lg font-semibold text-base-100 outline-none placeholder:text-base-500"
      placeholder="Notebook title..."
    />
    <div class="flex items-center gap-2">
      {#if saveStatus}
        <span class="text-[10px] text-green-400">{saveStatus}</span>
      {:else if dirty}
        <span class="text-[10px] text-yellow-400">Unsaved</span>
      {/if}
      <Button size="sm" variant="primary" onclick={save} disabled={saving}>
        {saving ? "Saving..." : "Save"}
      </Button>
    </div>
  </div>

  <div class="flex items-center gap-0.5 border-b border-base-700 px-3 py-1.5 flex-wrap">
    <button onclick={() => execCmd("bold")} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Bold (Ctrl+B)">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"/><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"/></svg>
    </button>
    <button onclick={() => execCmd("italic")} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Italic (Ctrl+I)">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="19" y1="4" x2="10" y2="4"/><line x1="14" y1="20" x2="5" y2="20"/><line x1="15" y1="4" x2="9" y2="20"/></svg>
    </button>
    <button onclick={() => execCmd("underline")} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Underline (Ctrl+U)">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3"/><line x1="4" y1="21" x2="20" y2="21"/></svg>
    </button>
    <div class="w-px h-4 bg-base-600 mx-1"></div>
    <button onclick={() => execCmd("formatBlock", "h2")} class="rounded px-1.5 py-0.5 text-[10px] font-bold text-base-400 hover:bg-base-700 hover:text-base-200" title="Heading">
      H2
    </button>
    <button onclick={() => execCmd("formatBlock", "h3")} class="rounded px-1.5 py-0.5 text-[10px] font-bold text-base-400 hover:bg-base-700 hover:text-base-200" title="Subheading">
      H3
    </button>
    <button onclick={() => execCmd("formatBlock", "p")} class="rounded px-1.5 py-0.5 text-[10px] text-base-400 hover:bg-base-700 hover:text-base-200" title="Paragraph">
      P
    </button>
    <div class="w-px h-4 bg-base-600 mx-1"></div>
    <button onclick={() => execCmd("insertUnorderedList")} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Bullet list">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><circle cx="3" cy="6" r="1" fill="currentColor"/><circle cx="3" cy="12" r="1" fill="currentColor"/><circle cx="3" cy="18" r="1" fill="currentColor"/></svg>
    </button>
    <button onclick={() => execCmd("insertOrderedList")} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Numbered list">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="10" y1="6" x2="21" y2="6"/><line x1="10" y1="12" x2="21" y2="12"/><line x1="10" y1="18" x2="21" y2="18"/><text x="2" y="8" font-size="8" fill="currentColor" stroke="none">1</text><text x="2" y="14" font-size="8" fill="currentColor" stroke="none">2</text><text x="2" y="20" font-size="8" fill="currentColor" stroke="none">3</text></svg>
    </button>
    <div class="w-px h-4 bg-base-600 mx-1"></div>
    <button onclick={insertHr} class="rounded p-1 text-base-400 hover:bg-base-700 hover:text-base-200" title="Horizontal rule">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="2" y1="12" x2="22" y2="12"/></svg>
    </button>
    <button onclick={insertTimestamp} class="rounded px-1.5 py-0.5 text-[10px] text-base-400 hover:bg-base-700 hover:text-base-200" title="Insert timestamp">
      ⏱ Time
    </button>
    <div class="w-px h-4 bg-base-600 mx-1"></div>
    <span class="text-[9px] text-base-500">Paste images with Ctrl+V</span>
  </div>

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    bind:this={editorEl}
    contenteditable="true"
    oninput={handleInput}
    onpaste={handlePaste}
    class="flex-1 overflow-y-auto p-6 text-sm text-base-200 leading-relaxed outline-none notebook-content"
    style="max-width: 800px; margin: 0 auto; width: 100%;"
  ></div>
</div>

<style>
  :global(.notebook-content h2) {
    font-size: 1.25rem;
    font-weight: 700;
    color: #e8e9f2;
    margin: 1rem 0 0.5rem;
  }
  :global(.notebook-content h3) {
    font-size: 1rem;
    font-weight: 600;
    color: #c4c7df;
    margin: 0.75rem 0 0.25rem;
  }
  :global(.notebook-content p) {
    margin: 0.25rem 0;
  }
  :global(.notebook-content ul),
  :global(.notebook-content ol) {
    padding-left: 1.5rem;
    margin: 0.25rem 0;
  }
  :global(.notebook-content li) {
    margin: 0.125rem 0;
  }
  :global(.notebook-content hr) {
    border-color: #3a3d52;
    margin: 0.75rem 0;
  }
  :global(.notebook-content img) {
    max-width: 100%;
    border-radius: 0.5rem;
    border: 1px solid #3a3d52;
    margin: 0.5rem 0;
  }
  :global(.notebook-content strong) {
    color: #e8e9f2;
    font-weight: 600;
  }
  :global(.notebook-content em) {
    color: #9498b8;
  }
</style>
