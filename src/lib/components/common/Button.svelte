<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "primary" | "secondary" | "ghost" | "danger";
    size?: "sm" | "md" | "lg";
    disabled?: boolean;
    onclick?: () => void;
    children: Snippet;
  }

  let { variant = "secondary", size = "md", disabled = false, onclick, children }: Props = $props();

  const baseClass = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-accent/50 disabled:opacity-50 disabled:cursor-not-allowed";

  const variants = {
    primary: "bg-accent text-white hover:bg-accent-hover",
    secondary: "border border-base-600 bg-base-700 text-base-200 hover:bg-base-600",
    ghost: "text-base-300 hover:bg-base-700 hover:text-base-100",
    danger: "bg-red-600 text-white hover:bg-red-500",
  };

  const sizes = {
    sm: "px-2 py-1 text-xs gap-1",
    md: "px-3 py-1.5 text-sm gap-2",
    lg: "px-4 py-2 text-base gap-2",
  };
</script>

<button
  class="{baseClass} {variants[variant]} {sizes[size]}"
  {disabled}
  {onclick}
>
  {@render children()}
</button>
