<script lang="ts">
  import EditorShell from "../../lib/components/editor/EditorShell.svelte";
  import PartsLibrary from "../../lib/components/parts/PartsLibrary.svelte";
  import ToolsSidebar from "../../lib/components/tools/ToolsSidebar.svelte";
  import ChatPanel from "../../lib/components/ai/ChatPanel.svelte";
  import CompilerWizard from "../../lib/components/ai/CompilerWizard.svelte";
  import SimulatorPanel from "../../lib/components/ai/SimulatorPanel.svelte";
  import { rightSidebarOpen, rightSidebarTab } from "../../lib/stores/ui.svelte";
</script>

<div class="flex h-full overflow-hidden">
  <div class="w-64 flex-shrink-0 border-r border-base-700 overflow-hidden">
    <PartsLibrary />
  </div>

  <div class="flex-1 overflow-hidden">
    <EditorShell />
  </div>

  {#if rightSidebarOpen.value}
    <div class="w-80 flex-shrink-0 border-l border-base-700 flex flex-col overflow-hidden">
      <div class="flex border-b border-base-700">
        {#each [["tools", "Tools"], ["chat", "AI Chat"], ["compiler", "Compiler"], ["simulator", "Simulator"]] as [tab, label]}
          <button
            onclick={() => rightSidebarTab.value = tab as typeof rightSidebarTab.value}
            class="flex-1 px-2 py-2 text-xs font-medium transition-colors
              {rightSidebarTab.value === tab
                ? 'border-b-2 border-accent text-accent'
                : 'text-base-400 hover:text-base-200'}"
          >
            {label}
          </button>
        {/each}
      </div>

      <div class="flex-1 overflow-y-auto">
        {#if rightSidebarTab.value === "tools"}
          <ToolsSidebar />
        {:else if rightSidebarTab.value === "chat"}
          <ChatPanel />
        {:else if rightSidebarTab.value === "compiler"}
          <CompilerWizard />
        {:else if rightSidebarTab.value === "simulator"}
          <SimulatorPanel />
        {/if}
      </div>
    </div>
  {/if}
</div>
