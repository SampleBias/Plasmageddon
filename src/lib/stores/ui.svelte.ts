export const currentRoute = $state({ value: "/" });
export const showCommandPalette = $state({ value: false });
export const activeSidebarItem = $state({ value: "home" });
export const rightSidebarOpen = $state({ value: true });
export const rightSidebarTab = $state({ value: "tools" as "tools" | "chat" | "compiler" | "simulator" });
export const activeEditorView = $state({ value: "schematic" as "schematic" | "sequence" | "circular" | "split" });

export function navigate(path: string) {
  currentRoute.value = path;
  if (path === "/") activeSidebarItem.value = "home";
  else if (path === "/repos") activeSidebarItem.value = "repos";
  else if (path.startsWith("/editor")) activeSidebarItem.value = "editor";
  else if (path === "/settings") activeSidebarItem.value = "settings";
  else if (path === "/chat") activeSidebarItem.value = "chat";
}
