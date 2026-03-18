<script lang="ts">
  import { settings } from "../../lib/api/tauri";
  import Button from "../../lib/components/common/Button.svelte";

  let glm5Key = $state("");
  let groqKey = $state("");
  let saved = $state(false);

  async function loadSettings() {
    try {
      glm5Key = (await settings.get("glm5_api_key")) ?? "";
      groqKey = (await settings.get("groq_api_key")) ?? "";
    } catch {}
  }

  async function saveSettings() {
    try {
      await settings.set("glm5_api_key", glm5Key);
      await settings.set("groq_api_key", groqKey);
      saved = true;
      setTimeout(() => (saved = false), 3000);
    } catch {}
  }

  $effect(() => { loadSettings(); });
</script>

<div class="mx-auto max-w-2xl p-8">
  <h1 class="mb-6 text-2xl font-bold text-base-100">Settings</h1>

  <div class="space-y-8">
    <section class="rounded-xl border border-base-700 bg-base-800 p-6">
      <h2 class="mb-4 text-sm font-semibold text-base-200">API Keys</h2>
      <p class="mb-4 text-xs text-base-400">
        AI features require API keys. Keys are stored locally in your database — never transmitted anywhere except the respective API endpoints.
      </p>

      <div class="space-y-4">
        <div>
          <label for="glm5-key" class="mb-1 block text-xs font-medium text-base-300">
            GLM-5 / Zhipu AI API Key
          </label>
          <p class="mb-2 text-xs text-base-500">Used for Compiler, Simulator, and AI Chat</p>
          <input
            id="glm5-key"
            type="password"
            bind:value={glm5Key}
            class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent font-mono"
            placeholder="Enter your GLM-5 API key"
          />
        </div>

        <div>
          <label for="groq-key" class="mb-1 block text-xs font-medium text-base-300">
            Groq Cloud API Key
          </label>
          <p class="mb-2 text-xs text-base-500">Used for Part Match and lightweight tasks</p>
          <input
            id="groq-key"
            type="password"
            bind:value={groqKey}
            class="w-full rounded-md border border-base-600 bg-base-700 px-3 py-2 text-sm text-base-100 outline-none focus:border-accent font-mono"
            placeholder="Enter your Groq API key"
          />
        </div>
      </div>

      <div class="mt-6 flex items-center gap-4">
        <Button variant="primary" onclick={saveSettings}>Save Settings</Button>
        {#if saved}
          <span class="text-sm text-green-400">Saved</span>
        {/if}
      </div>
    </section>

    <section class="rounded-xl border border-base-700 bg-base-800 p-6">
      <h2 class="mb-2 text-sm font-semibold text-base-200">About</h2>
      <p class="text-xs text-base-400">
        <strong class="text-base-200">Plasmageddon</strong> v0.1.0 — End of bad designs<br/>
        Native desktop CAD for synthetic biology. Built with Tauri, Svelte, and Rust.
      </p>
    </section>
  </div>
</div>
