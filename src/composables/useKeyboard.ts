import { onMounted, onBeforeUnmount } from "vue";

export type PaneMode = "both" | "editor" | "preview";

export interface KeyboardActions {
  onOpenFile: () => void;
  onOpenFolder: () => void;
  onExportPdf: () => void;
  onTogglePane: (mode: PaneMode) => void;
  onCycleTheme: () => void;
}

const isMac = navigator.platform.toLowerCase().includes("mac");

export function useKeyboard(actions: KeyboardActions) {
  function isMod(e: KeyboardEvent): boolean {
    return isMac ? e.metaKey : e.ctrlKey;
  }

  function onKeyDown(e: KeyboardEvent) {
    if (!isMod(e)) return;

    const key = e.key.toLowerCase();

    switch (key) {
      case "o":
        if (e.shiftKey) {
          e.preventDefault();
          actions.onOpenFolder();
        } else {
          e.preventDefault();
          actions.onOpenFile();
        }
        break;
      case "s":
        e.preventDefault();
        actions.onExportPdf();
        break;
      case "e":
        e.preventDefault();
        actions.onTogglePane("editor");
        break;
      case "p":
        e.preventDefault();
        actions.onTogglePane("preview");
        break;
      case "b":
        e.preventDefault();
        actions.onTogglePane("both");
        break;
      case "t":
        e.preventDefault();
        actions.onCycleTheme();
        break;
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", onKeyDown);
  });

  onBeforeUnmount(() => {
    window.removeEventListener("keydown", onKeyDown);
  });
}
