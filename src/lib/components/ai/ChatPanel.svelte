<script lang="ts">
  import { chatMessages, chatLoading, chatInput } from "../../stores/chat.svelte";
  import { currentConstruct } from "../../stores/construct.svelte";
  import { ai, events } from "../../api/tauri";
  import Button from "../common/Button.svelte";

  let messagesEl: HTMLDivElement;
  let streamingContent = $state("");
  let isStreaming = $state(false);

  async function loadHistory() {
    try {
      const constructId = currentConstruct.value?.id ?? null;
      const history = await ai.getChatHistory(constructId);
      chatMessages.value = history;
    } catch {}
  }

  async function sendMessage() {
    const msg = chatInput.value.trim();
    if (!msg || chatLoading.value) return;

    const constructId = currentConstruct.value?.id ?? null;
    chatInput.value = "";
    chatLoading.value = true;
    isStreaming = true;
    streamingContent = "";

    chatMessages.value = [
      ...chatMessages.value,
      { id: "temp-user", construct_id: constructId, role: "user", content: msg, created_at: "" },
    ];

    try {
      const unlistenChunk = await events.onAiChunk((chunk) => {
        streamingContent += chunk;
      });

      const unlistenDone = await events.onAiDone(() => {
        isStreaming = false;
        unlistenChunk();
        unlistenDone();
      });

      await ai.chatStream(constructId, msg);
      await loadHistory();
    } catch (e) {
      chatMessages.value = [
        ...chatMessages.value,
        {
          id: "temp-error",
          construct_id: constructId,
          role: "assistant",
          content: `Error: ${e}`,
          created_at: "",
        },
      ];
    }

    chatLoading.value = false;
    isStreaming = false;
    streamingContent = "";
  }

  async function clearChat() {
    try {
      await ai.clearChat(currentConstruct.value?.id ?? null);
      chatMessages.value = [];
    } catch {}
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }

  $effect(() => {
    currentConstruct.value;
    loadHistory();
  });

  $effect(() => {
    chatMessages.value;
    if (messagesEl) {
      messagesEl.scrollTop = messagesEl.scrollHeight;
    }
  });
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-base-700 px-3 py-2">
    <span class="text-xs font-semibold text-base-300">
      Kernel AI
      {#if currentConstruct.value}
        <span class="text-base-500">· {currentConstruct.value.name}</span>
      {/if}
    </span>
    <button onclick={clearChat} class="text-[10px] text-base-400 hover:text-base-200">Clear</button>
  </div>

  <div bind:this={messagesEl} class="flex-1 overflow-y-auto p-3 space-y-3">
    {#each chatMessages.value.filter((m) => m.role !== "system") as msg}
      <div class="flex flex-col {msg.role === 'user' ? 'items-end' : 'items-start'}">
        <div
          class="max-w-[90%] rounded-lg px-3 py-2 text-xs leading-relaxed
            {msg.role === 'user'
              ? 'bg-accent/20 text-accent'
              : 'bg-base-700 text-base-200'}"
        >
          <pre class="whitespace-pre-wrap font-sans">{msg.content}</pre>
        </div>
      </div>
    {/each}

    {#if isStreaming && streamingContent}
      <div class="flex flex-col items-start">
        <div class="max-w-[90%] rounded-lg bg-base-700 px-3 py-2 text-xs text-base-200 leading-relaxed">
          <pre class="whitespace-pre-wrap font-sans">{streamingContent}</pre>
          <span class="inline-block w-1.5 h-3 bg-accent animate-pulse ml-0.5"></span>
        </div>
      </div>
    {/if}

    {#if chatMessages.value.length === 0 && !isStreaming}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <p class="text-xs text-base-400 mb-1">Ask Kernel AI anything about your design</p>
          <p class="text-[10px] text-base-500">Suggest parts, explain biology, optimize constructs</p>
        </div>
      </div>
    {/if}
  </div>

  <div class="border-t border-base-700 p-3">
    <div class="flex gap-2">
      <textarea
        bind:value={chatInput.value}
        onkeydown={handleKeydown}
        placeholder="Ask about your construct..."
        rows={2}
        class="flex-1 resize-none rounded-md border border-base-600 bg-base-700 px-3 py-2 text-xs text-base-100 outline-none focus:border-accent placeholder:text-base-500"
      ></textarea>
      <Button variant="primary" size="sm" onclick={sendMessage} disabled={chatLoading.value || !chatInput.value.trim()}>
        Send
      </Button>
    </div>
  </div>
</div>
