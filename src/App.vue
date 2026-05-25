<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";

import { useTheme } from "./composables/useTheme";
import { usePdf } from "./composables/usePdf";
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

const hasContent = computed(() => editorContent.value.trim().length > 0);

// ─── Theme ───
const { theme, setTheme } = useTheme();

// ─── PDF ───
const { pdfUrl, pdfLoading, lastError } = usePdf(editorContent, selectedTemplate);

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

    const files = await invoke<[string, string][]>("open_folder", { folderPath: path });
    folderFiles.value = files.map(([name, content]) => ({ name, content }));
    currentFolderPath.value = path;

    // Load the first .md file (or one named index.md / main.md if present)
    const entry =
      folderFiles.value.find(
        (f) => f.name.toLowerCase() === "index.md" || f.name.toLowerCase() === "main.md"
      ) || folderFiles.value[0];

    if (entry) {
      editorContent.value = entry.content;
      isWelcome.value = false;
    }
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
          <EditorPane v-model="editorContent" :theme="theme" />
        </template>
        <template #preview>
          <PreviewPane :pdf-url="pdfUrl" :loading="pdfLoading" :error="lastError" />
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
