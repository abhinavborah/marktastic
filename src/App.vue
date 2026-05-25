<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";

import { useTheme } from "./composables/useTheme";
import { usePdf } from "./composables/usePdf";
import { useScrollSync } from "./composables/useScrollSync";
import Toolbar from "./components/Toolbar.vue";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import SplitView from "./components/SplitView.vue";
import EditorPane from "./components/EditorPane.vue";
import PreviewPane from "./components/PreviewPane.vue";

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

const hasContent = computed(() => editorContent.value.trim().length > 0);

// ─── Theme ───
const { theme, setTheme } = useTheme();

// ─── PDF ───
// Always uses convert_md_to_pdf with the current editorContent as the single source of truth.
// In folder mode, the merged editor content is passed directly — edits are reflected in the preview.
const { pdfUrl, pdfLoading, lastError } = usePdf(editorContent, selectedTemplate);

// ─── Scroll Sync ───
const editorViewRef = ref<any>(null);
const previewIframeRef = ref<HTMLIFrameElement | null>(null);
useScrollSync(editorViewRef, previewIframeRef);

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
  } catch (err) {
    console.error("Failed to open file:", err);
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
    try {
      const ordered = await invoke<string[]>("resolve_wikilinks", { folderPath: path });
      reachableFiles.value = ordered;
    } catch (err) {
      console.error("Wikilink resolution failed:", err);
      // Fallback: include all .md files
      reachableFiles.value = files.map(([name]) => name);
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
    // usePdf watches editorContent and will call convert_md_to_pdf with the merged content
    isWelcome.value = false;
  } catch (err) {
    console.error("Failed to open folder:", err);
  }
}

// ─── Export PDF ───
async function handleExportPdf() {
  if (!pdfUrl.value) return;

  try {
    const path = await save({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      defaultPath: "document.pdf",
    });
    if (!path || typeof path !== "string") return;

    // Fetch the blob and convert to Uint8Array
    const response = await fetch(pdfUrl.value);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    await writeFile(path, uint8Array);
  } catch (err) {
    console.error("Failed to export PDF:", err);
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
      <SplitView v-else>
        <template #editor>
          <EditorPane
            v-model="editorContent"
            :theme="theme"
            @editor-ready="(v: any) => editorViewRef = v"
          />
        </template>
        <template #preview>
          <PreviewPane
            :pdf-url="pdfUrl"
            :loading="pdfLoading"
            :error="lastError"
            @iframe-ready="(el: HTMLIFrameElement) => previewIframeRef = el"
          />
        </template>
      </SplitView>
    </main>
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
