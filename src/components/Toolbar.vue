<script setup lang="ts">
import { computed } from "vue";
import type { ThemeMode } from "../types";

const props = defineProps<{
  theme: ThemeMode;
  templates: string[];
  selectedTemplate: string;
  hasContent: boolean;
  folderPath?: string | null;
  reachableFiles?: string[];
}>();

const emit = defineEmits<{
  (e: "update:theme", theme: ThemeMode): void;
  (e: "update:selectedTemplate", template: string): void;
  (e: "openFile"): void;
  (e: "openFolder"): void;
  (e: "exportPdf"): void;
}>();

const themeLabel = computed(() => {
  switch (props.theme) {
    case "light":
      return "☀ Light";
    case "dark":
      return "☾ Dark";
    case "system":
      return "◐ System";
  }
});

function cycleTheme() {
  const modes: ThemeMode[] = ["light", "dark", "system"];
  const idx = modes.indexOf(props.theme);
  const next = modes[(idx + 1) % modes.length];
  emit("update:theme", next);
}

const folderName = computed(() => {
  if (!props.folderPath) return null;
  const parts = props.folderPath.split(/[/\\]/);
  return parts[parts.length - 1] || parts[parts.length - 2] || props.folderPath;
});

const isMac = navigator.platform.toLowerCase().includes("mac");
const mod = isMac ? "⌘" : "Ctrl";
</script>

<template>
  <header
    class="flex items-center gap-3 px-4 py-2 border-b bg-background z-30 shrink-0"
    :class="isMac ? 'pl-20' : ''"
  >
    <!-- Logo / Title -->
    <div class="flex items-center gap-2 mr-2 select-none">
      <svg
        class="w-6 h-6 text-primary"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
        <polyline points="14 2 14 8 20 8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <line x1="10" y1="9" x2="8" y2="9" />
      </svg>
      <span class="font-bold text-lg tracking-tight">Marktastic</span>
    </div>

    <!-- Folder indicator -->
    <div
      v-if="folderName"
      class="flex items-center gap-1.5 px-2 py-1 text-xs rounded bg-muted text-muted-foreground"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
      </svg>
      <span class="font-medium">{{ folderName }}</span>
      <span v-if="reachableFiles && reachableFiles.length > 0" class="opacity-70">
        ({{ reachableFiles.length }} files)
      </span>
    </div>

    <div class="flex-1" />

    <!-- Open buttons -->
    <button
      :title="`Open File (${mod}+O)`"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/90 transition-colors"
      @click="$emit('openFile')"
    >
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg>
      Open
      <span class="hidden sm:inline text-xs opacity-60">{{ mod }}+O</span>
    </button>
    <button
      :title="`Open Folder (${mod}+Shift+O)`"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/90 transition-colors"
      @click="$emit('openFolder')"
    >
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
      Folder
      <span class="hidden sm:inline text-xs opacity-60">{{ mod }}+⇧+O</span>
    </button>

    <div class="w-px h-6 bg-border mx-1" />

    <!-- Template selector -->
    <div class="flex items-center gap-1.5" :title="`Template for PDF export`">
      <label class="text-sm text-muted-foreground">Template:</label>
      <select
        class="px-2 py-1.5 text-sm rounded-md border bg-background text-foreground focus:outline-none focus:ring-1 focus:ring-ring transition-colors"
        :value="selectedTemplate"
        @change="$emit('update:selectedTemplate', ($event.target as HTMLSelectElement).value)"
      >
        <option v-for="t in templates" :key="t" :value="t">
          {{ t.replace(/-/g, " ") }}
        </option>
      </select>
    </div>

    <div class="w-px h-6 bg-border mx-1" />

    <!-- Theme toggle -->
    <button
      :title="`Cycle theme: ${mod}+T`"
      class="px-3 py-1.5 text-sm font-medium rounded-md border hover:bg-muted transition-colors"
      @click="cycleTheme"
    >
      {{ themeLabel }}
    </button>

    <!-- Export PDF -->
    <button
      :title="`Export PDF (${mod}+S)`"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      :disabled="!hasContent"
      @click="$emit('exportPdf')"
    >
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      Export
      <span class="hidden sm:inline text-xs opacity-60">{{ mod }}+S</span>
    </button>
  </header>
</template>
