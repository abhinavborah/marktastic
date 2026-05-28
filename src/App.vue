<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

import { useTheme } from "./composables/useTheme";
import { useSvgRenderer } from "./composables/useSvgRenderer";
import { useToast } from "./composables/useToast";
import { useKeyboard } from "./composables/useKeyboard";
import { useTemplates } from "./composables/useTemplates";
import type { PaneMode } from "./composables/useKeyboard";

import Toolbar from "./components/Toolbar.vue";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import SplitView from "./components/SplitView.vue";
import EditorPane from "./components/EditorPane.vue";
import PreviewPane from "./components/PreviewPane.vue";
import TemplateManager from "./components/TemplateManager.vue";

// ─── State ───
const editorContent = ref("");
const selectedTemplate = ref("basic-report");
const isWelcome = ref(true);
const currentFilePath = ref<string | null>(null);
const currentFolderPath = ref<string | null>(null);
const folderFiles = ref<{ name: string; content: string }[]>([]);
const reachableFiles = ref<string[]>([]);
const isFolderMode = ref(false);
const paneMode = ref<PaneMode>("both");
const wordWrap = ref(false);
const zoomLevel = ref(1.0);
const showTemplateManager = ref(false);

const hasContent = computed(() => editorContent.value.trim().length > 0);

// ─── Theme ───
const { theme, setTheme, cycleTheme } = useTheme();

// ─── Toast ───
const toast = useToast();

// ─── SVG Preview Renderer ───
const { pages, totalPages, rendering: svgRendering, renderError, isRecompiling } = useSvgRenderer(
  editorContent,
  selectedTemplate
);

// ─── Templates ───
const { templates, refreshTemplates } = useTemplates();

// Show "Recompiling..." toast while SVG is recompiling
const recompileToastId = ref<number | null>(null);
watch(isRecompiling, (val) => {
  if (val) {
    recompileToastId.value = toast.loading("Recompiling...", 0);
  } else if (recompileToastId.value !== null) {
    toast.dismiss(recompileToastId.value);
    recompileToastId.value = null;
  }
});

const previewLoading = computed(() => svgRendering.value);

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
  await refreshTemplates();
  if (templates.value.length > 0 && !templates.value.some(t => t.name === selectedTemplate.value)) {
    selectedTemplate.value = templates.value[0].name;
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

// ─── Export PDF ───
async function handleExportPdf() {
  const path = await save({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      defaultPath: "document.pdf",
    });
    if (!path || typeof path !== "string") {
      return;
    }


    try {
      toast.info("Compiling PDF...", 0);
      const pdfBytes = await invoke<number[]>("convert_md_to_pdf", {
        markdown: editorContent.value,
        templateName: selectedTemplate.value,
      });
      await writeFile(path, new Uint8Array(pdfBytes));
      toast.success(`Saved to ${path.split(/[/\\]/).pop() || path}`);
    } catch (err: any) {
      toast.error(`Export failed: ${err?.message || String(err)}`);
    }
}
</script>

<template>
  <div class="flex flex-col h-screen bg-background text-foreground overflow-hidden">
    <!-- Toolbar -->
    <Toolbar
      :theme="theme"
      :templates="templates.map(t => t.name)"
      :selected-template="selectedTemplate"
      :has-content="hasContent"
      :folder-path="currentFolderPath"
      :reachable-files="reachableFiles"
      @update:theme="setTheme"
      @update:selected-template="selectedTemplate = $event"
      @open-file="handleOpenFile"
      @open-folder="handleOpenFolder"
      @export-pdf="handleExportPdf"
      @open-template-manager="showTemplateManager = true"
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
            :total-pages="totalPages"
            :rendering="previewLoading"
            :error="renderError"
            :zoom="zoomLevel"
          />
        </template>
      </SplitView>
    </main>

    <!-- Template Manager Modal -->
    <TemplateManager v-if="showTemplateManager" @close="showTemplateManager = false" />
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
