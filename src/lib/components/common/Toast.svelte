<script lang="ts">
  interface Props {
    message: string;
    type?: "info" | "success" | "error" | "warning";
    show: boolean;
    ondismiss: () => void;
  }

  let { message, type = "info", show, ondismiss }: Props = $props();

  const colors = {
    info: "border-accent bg-accent/10 text-accent",
    success: "border-green-500 bg-green-500/10 text-green-400",
    error: "border-red-500 bg-red-500/10 text-red-400",
    warning: "border-yellow-500 bg-yellow-500/10 text-yellow-400",
  };

  $effect(() => {
    if (show) {
      const timer = setTimeout(ondismiss, 4000);
      return () => clearTimeout(timer);
    }
  });
</script>

{#if show}
  <div class="fixed bottom-4 right-4 z-50 animate-in fade-in slide-in-from-bottom-4">
    <div class="flex items-center gap-3 rounded-lg border px-4 py-3 shadow-lg {colors[type]}">
      <span class="text-sm">{message}</span>
      <button onclick={ondismiss} class="text-current opacity-60 hover:opacity-100">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>
  </div>
{/if}
