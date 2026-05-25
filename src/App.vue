<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { open as openInShell } from "@tauri-apps/plugin-shell";
import { join, tempDir } from "@tauri-apps/api/path";
import { writeFile } from "@tauri-apps/plugin-fs";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

import { useTheme } from "./composables/useTheme";
import { usePdf } from "./composables/usePdf";
import { usePdfRenderer } from "./composables/usePdfRenderer";
import { useToast } from "./composables/useToast";
import { useKeyboard } from "./composables/useKeyboard";
import type { PaneMode } from "./composables/useKeyboard";

import Toolbar from "./components/Toolbar.vue";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import SplitView from "./components/SplitView.vue";
import EditorPane from "./components/EditorPane.vue";
import PreviewPane from "./components/PreviewPane.vue";
import ToastContainer from "./components/ToastContainer.vue";

// ─── State ───
const editorContent = ref("");
const selectedTemplate = ref("basic-report");
const templates = ref<string[]>([]);
const isWelcome = ref(true);
const currentFilePath = ref<string | null>(null);
const currentFolderPath = ref<string | null>(null);
const folderFiles = ref<{ name: string; content: string }[]>([]);
const reachableFiles = ref<string[]>([]);
const isFolderMode = ref(false);
const paneMode = ref<PaneMode>("both");
const wordWrap = ref(false);
const zoomLevel = ref(1.0);

const hasContent = computed(() => editorContent.value.trim().length > 0);

// ─── Theme ───
const { theme, setTheme, cycleTheme } = useTheme();

// ─── Toast ───
const toast = useToast();

// ─── PDF ───
const { pdfBytes, pdfLoading, lastError } = usePdf(
  editorContent,
  selectedTemplate,
  toast
);

// ─── PDF Image Renderer ───
const { pages, rendering: imageRendering, renderError } = usePdfRenderer(
  pdfBytes,
  zoomLevel
);

const previewLoading = computed(() => pdfLoading.value || imageRendering.value);
const previewError = computed(() => lastError.value || renderError.value);

// ─── Keyboard Shortcuts ───
useKeyboard({
  onOpenFile: () => handleOpenFile(),
  onOpenFolder: () => handleOpenFolder(),
  onExportPdf: () => handleExportPdf(),
  onTogglePane: (mode: PaneMode) => {
    paneMode.value = mode;
  },
  onCycleTheme: () => cycleTheme(),
  onToggleWordWrap: () => {
    wordWrap.value = !wordWrap.value;
  },
});

// ─── Window Title ───
async function updateWindowTitle() {
  try {
    const appWindow = getCurrentWebviewWindow();
    if (currentFilePath.value) {
      const fileName = currentFilePath.value.split(/[/\\]/).pop() || "untitled";
      await appWindow.setTitle(`Marktastic — ${fileName}`);
    } else if (currentFolderPath.value) {
      const folderName = currentFolderPath.value.split(/[/\\]/).pop() || "folder";
      await appWindow.setTitle(`Marktastic — ${folderName}`);
    } else {
      await appWindow.setTitle("Marktastic");
    }
  } catch {
    // webviewWindow API may not be available in all contexts
  }
}

// ─── Load templates on mount ───
onMounted(async () => {
  try {
    const list = await invoke<string[]>("get_templates");
    templates.value = list;
    if (list.length > 0 && !list.includes(selectedTemplate.value)) {
      selectedTemplate.value = list[0];
    }
  } catch (err) {
    console.error("Failed to load templates:", err);
    templates.value = ["basic-report", "university-assignment", "thesis-chapter"];
  }
});

// ─── File / Folder open ───
async function handleOpenFile() {
  try {
    const path = await open({
      multiple: false,
      filters: [{ name: "Markdown", extensions: ["md", "markdown"] }],
    });
    if (!path || typeof path !== "string") return;

    const content = await invoke<string>("open_file_path", { filePath: path });
    editorContent.value = content;
    currentFilePath.value = path;
    currentFolderPath.value = null;
    folderFiles.value = [];
    reachableFiles.value = [];
    isFolderMode.value = false;
    isWelcome.value = false;
    paneMode.value = "both";

    const fileName = path.split(/[/\\]/).pop() || path;
    toast.success(`Opened ${fileName}`);
    await updateWindowTitle();
  } catch (err) {
    console.error("Failed to open file:", err);
    toast.error("Failed to open file");
  }
}

async function handleOpenFolder() {
  try {
    const path = await open({
      directory: true,
    });
    if (!path || typeof path !== "string") return;

    // Get all files in the folder
    const files = await invoke<[string, string][]>("open_folder", { folderPath: path });
    folderFiles.value = files.map(([name, content]) => ({ name, content }));
    currentFolderPath.value = path;

    // Resolve wikilinks to get the ordered list of reachable files
    let brokenLinks = 0;
    try {
      const ordered = await invoke<string[]>("resolve_wikilinks", { folderPath: path });
      reachableFiles.value = ordered;
    } catch (err) {
      console.error("Wikilink resolution failed:", err);
      reachableFiles.value = files.map(([name]) => name);
    }

    // Check for broken wikilinks by comparing all links in entry file against reachable set
    const reachableSet = new Set(reachableFiles.value);
    const entryFile = folderFiles.value.find(
      (f) => f.name.toLowerCase() === "index.md" || f.name.toLowerCase() === "main.md"
    ) || folderFiles.value[0];
    if (entryFile) {
      const linkMatches = entryFile.content.match(/\[\[([^\]]+)\]\]/g);
      if (linkMatches) {
        for (const match of linkMatches) {
          const target = match.slice(2, -2).trim();
          const targetFile = target.endsWith(".md") ? target : `${target}.md`;
          if (!reachableSet.has(targetFile)) {
            brokenLinks++;
          }
        }
      }
    }

    // Build merged editor content from reachable files
    const fileMap = new Map(folderFiles.value.map((f) => [f.name, f.content]));
    const parts: string[] = [];
    for (let i = 0; i < reachableFiles.value.length; i++) {
      const name = reachableFiles.value[i];
      const content = fileMap.get(name) || "";
      if (i > 0) {
        parts.push("\n\n---\n\n");
      }
      parts.push(`<!-- file: ${name} -->\n\n${content}`);
    }
    editorContent.value = parts.join("");
    isFolderMode.value = true;
    isWelcome.value = false;
    paneMode.value = "both";

    const folderName = path.split(/[/\\]/).pop() || path;
    toast.success(`Opened folder "${folderName}" with ${reachableFiles.value.length} files`);
    if (brokenLinks > 0) {
      toast.warning(`${brokenLinks} broken wikilink${brokenLinks > 1 ? "s" : ""} found`);
    }
    await updateWindowTitle();
  } catch (err) {
    console.error("Failed to open folder:", err);
    toast.error("Failed to open folder");
  }
}

// ─── Zoom ───
function handleZoomIn() {
  console.log("Zoom in clicked, current:", zoomLevel.value);
  zoomLevel.value = Math.min(3, zoomLevel.value * 1.1);
  console.log("Zoom in new value:", zoomLevel.value);
}

function handleZoomOut() {
  console.log("Zoom out clicked, current:", zoomLevel.value);
  zoomLevel.value = Math.max(0.5, zoomLevel.value / 1.1);
  console.log("Zoom out new value:", zoomLevel.value);
}

// ─── Open in Preview ───
async function openInPreview() {
  if (!pdfBytes.value || pdfBytes.value.length === 0) {
    toast.warning("No PDF to preview. Open a file first.");
    return;
  }
  try {
    const temp = await tempDir();
    const tempPath = await join(temp, "marktastic-preview.pdf");
    await writeFile(tempPath, pdfBytes.value);
    await openInShell(tempPath);
    toast.success("Opened PDF in system preview");
  } catch (err: any) {
    console.error("Failed to open preview:", err);
    toast.error(`Preview failed: ${err?.message || String(err)}`);
  }
}

// ─── Export PDF ───
async function handleExportPdf() {
  console.log("Export: checking pdfBytes...", pdfBytes.value?.length);
  if (!pdfBytes.value || pdfBytes.value.length === 0) {
    toast.warning("No PDF to export. Open a file first.");
    return;
  }

  try {
    toast.info("Preparing export...", 2000);
    console.log("Export: opening save dialog...");
    const path = await save({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      defaultPath: "document.pdf",
    });
    console.log("Export: save dialog returned:", path);
    if (!path || typeof path !== "string") {
      console.log("Export: no path selected, cancelling");
      return;
    }

    console.log("Export: writing file to", path, "bytes:", pdfBytes.value.length);
    await writeFile(path, pdfBytes.value);
    console.log("Export: file written successfully");
    toast.success(`Saved to ${path.split(/[/\\]/).pop() || path}`);
  } catch (err: any) {
    console.error("Export: FAILED with error:", err);
    console.error("Export: error message:", err?.message || String(err));
    toast.error(`Export failed: ${err?.message || String(err)}`);
  }
}
</script>

<template>
  <div class="flex flex-col h-screen bg-background text-foreground overflow-hidden">
    <!-- Toolbar -->
    <Toolbar
      :theme="theme"
      :templates="templates"
      :selected-template="selectedTemplate"
      :has-content="hasContent"
      :folder-path="currentFolderPath"
      :reachable-files="reachableFiles"
      @update:theme="setTheme"
      @update:selected-template="selectedTemplate = $event"
      @open-file="handleOpenFile"
      @open-folder="handleOpenFolder"
      @export-pdf="handleExportPdf"
    />

    <!-- Main area -->
    <main class="flex-1 overflow-hidden">
      <!-- Welcome screen -->
      <WelcomeScreen
        v-if="isWelcome"
        @open-file="handleOpenFile"
        @open-folder="handleOpenFolder"
      />

      <!-- Editor + Preview split view -->
      <SplitView
        v-else
        v-model="paneMode"
        :word-wrap="wordWrap"
        :zoom="zoomLevel"
        @toggle-word-wrap="wordWrap = !wordWrap"
        @zoom-in="handleZoomIn"
        @zoom-out="handleZoomOut"
        @open-in-preview="openInPreview"
      >
        <template #editor>
          <EditorPane
            v-model="editorContent"
            :theme="theme"
            :word-wrap="wordWrap"
          />
        </template>
        <template #preview>
          <PreviewPane
            :pages="pages"
            :rendering="previewLoading"
            :error="previewError"
          />
        </template>
      </SplitView>
    </main>

    <!-- Toast notifications -->
    <ToastContainer />
  </div>
</template>

<style>
/* Ensure full height */
html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

/* Remove default Tauri starter styles */
body {
  margin: 0;
}
</style>
