/**
 * ViewSync coordinates state between the three editor views.
 * The single source of truth is the construct store (construct.svelte.ts).
 * Each PixiJS view subscribes to the store's reactive state via Svelte $effect.
 *
 * Selection state flows:
 * - Clicking a part in Schematic or Circular sets selection.type = "part"
 * - Selecting bases in Sequence sets selection.type = "range"
 * - All three views read the selection store and highlight accordingly
 *
 * Mutation flow:
 * - Drag-drop / reorder in Schematic -> Tauri command -> refresh constructParts store
 * - All views re-render from the updated store
 */

export const VIEW_SYNC_VERSION = 1;
