<script lang="ts">
  import Header from "./lib/components/layout/Header.svelte";
  import Sidebar from "./lib/components/layout/Sidebar.svelte";
  import CommandPalette from "./lib/components/layout/CommandPalette.svelte";
  import { currentRoute, showCommandPalette } from "./lib/stores/ui.svelte";

  import HomePage from "./routes/HomePage.svelte";
  import ReposPage from "./routes/repos/ReposPage.svelte";
  import EditorPage from "./routes/editor/EditorPage.svelte";
  import SettingsPage from "./routes/settings/SettingsPage.svelte";

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      showCommandPalette.value = !showCommandPalette.value;
    }
    if (e.key === "Escape") {
      showCommandPalette.value = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-screen w-screen overflow-hidden bg-base-900">
  <Sidebar />
  <div class="flex flex-1 flex-col overflow-hidden">
    <Header />
    <main class="flex-1 overflow-auto">
      {#if currentRoute.value === "/"}
        <HomePage />
      {:else if currentRoute.value === "/repos"}
        <ReposPage />
      {:else if currentRoute.value.startsWith("/editor")}
        <EditorPage />
      {:else if currentRoute.value === "/settings"}
        <SettingsPage />
      {:else}
        <HomePage />
      {/if}
    </main>
  </div>
</div>

{#if showCommandPalette.value}
  <CommandPalette />
{/if}
