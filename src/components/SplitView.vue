<script setup lang="ts">
import { ref, computed, onBeforeUnmount } from "vue";

type PaneMode = "both" | "editor" | "preview";

const props = defineProps<{
  modelValue: PaneMode;
  wordWrap?: boolean;
  zoom?: number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", mode: PaneMode): void;
  (e: "toggleWordWrap"): void;
  (e: "zoomIn"): void;
  (e: "zoomOut"): void;
  (e: "revealInFinder"): void;
}>();

const paneMode = computed({
  get: () => props.modelValue,
  set: (val) => emit("update:modelValue", val),
});

const editorWidth = ref(50);
const isDragging = ref(false);

const showEditor = computed(() => paneMode.value !== "preview");
const showPreview = computed(() => paneMode.value !== "editor");
const isBoth = computed(() => paneMode.value === "both");

let dragCleanup: (() => void) | null = null;

function startDrag(e: MouseEvent) {
  if (!isBoth.value) return;
  isDragging.value = true;

  // Prevent text selection during drag
  document.body.style.userSelect = "none";
  document.body.style.cursor = "col-resize";

  const handleMouseMove = (ev: MouseEvent) => {
    if (!isDragging.value) return;
    const container = document.getElementById("split-container");
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const pct = ((ev.clientX - rect.left) / rect.width) * 100;
    editorWidth.value = Math.max(20, Math.min(80, pct));
  };

  const handleMouseUp = () => {
    isDragging.value = false;
    document.body.style.userSelect = "";
    document.body.style.cursor = "";
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", handleMouseUp);
    window.removeEventListener("blur", handleMouseUp);
    dragCleanup = null;
  };

  // Use window listeners so they fire even when mouse leaves the document
  // (e.g. over an iframe or outside the window)
  window.addEventListener("mousemove", handleMouseMove);
  window.addEventListener("mouseup", handleMouseUp);
  // Handle window losing focus (e.g. Alt+Tab during drag)
  window.addEventListener("blur", handleMouseUp);

  dragCleanup = handleMouseUp;
  e.preventDefault();
}

onBeforeUnmount(() => {
  if (dragCleanup) dragCleanup();
});

function togglePane(mode: PaneMode) {
  paneMode.value = mode;
}

const zoomPercent = computed(() => {
  const z = props.zoom ?? 1;
  return Math.round(z * 100) + "%";
});
</script>

<template>
  <div id="split-container" class="flex h-full relative" :data-dragging="isDragging">
    <!-- Editor pane -->
    <div
      v-if="showEditor"
      class="h-full overflow-hidden"
      :style="isBoth ? { width: editorWidth + '%' } : { width: '100%' }"
    >
      <slot name="editor" />
    </div>

    <!-- Drag handle -->
    <div
      v-if="isBoth"
      class="w-1 bg-border cursor-col-resize flex-shrink-0 hover:bg-ring transition-colors z-10"
      :class="{ 'bg-ring': isDragging }"
      @mousedown="startDrag"
    />

    <!-- Preview pane -->
    <div
      v-if="showPreview"
      class="h-full overflow-hidden"
      :style="isBoth ? { width: 100 - editorWidth + '%' } : { width: '100%' }"
    >
      <slot name="preview" />
    </div>

    <!-- Floating controls -->
    <div
      class="absolute top-2 right-2 flex gap-1 bg-card/90 backdrop-blur-sm border rounded-md p-1 shadow-sm z-20"
    >
      <!-- Pane mode buttons -->
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        :class="{ 'bg-muted': paneMode === 'editor' }"
        title="Editor only (⌘/Ctrl+E)"
        @click="togglePane('editor')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="9" y1="3" x2="9" y2="21" />
        </svg>
      </button>
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        :class="{ 'bg-muted': paneMode === 'both' }"
        title="Split view (⌘/Ctrl+B)"
        @click="togglePane('both')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="12" y1="3" x2="12" y2="21" />
        </svg>
      </button>
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        :class="{ 'bg-muted': paneMode === 'preview' }"
        title="Preview only (⌘/Ctrl+P)"
        @click="togglePane('preview')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="15" y1="3" x2="15" y2="21" />
        </svg>
      </button>

      <div class="w-px h-4 bg-border mx-0.5" />

      <!-- Word wrap -->
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        :class="{ 'bg-muted': wordWrap }"
        title="Toggle word wrap (⌘/Ctrl+Shift+W)"
        @click="$emit('toggleWordWrap')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M4 7h16M4 12h12M4 17h8" />
        </svg>
      </button>

      <div class="w-px h-4 bg-border mx-0.5" />

      <!-- Zoom out -->
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        title="Zoom out"
        @click="$emit('zoomOut')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
          <line x1="8" y1="11" x2="14" y2="11" />
        </svg>
      </button>

      <!-- Zoom percent -->
      <span class="flex items-center px-1 text-xs text-muted-foreground min-w-[3em] justify-center select-none">
        {{ zoomPercent }}
      </span>

      <!-- Zoom in -->
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        title="Zoom in"
        @click="$emit('zoomIn')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
          <line x1="11" y1="8" x2="11" y2="14" />
          <line x1="8" y1="11" x2="14" y2="11" />
        </svg>
      </button>

      <div class="w-px h-4 bg-border mx-0.5" />

      <!-- Reveal in Finder -->
      <button
        class="p-1.5 rounded hover:bg-muted transition-colors"
        title="Reveal in Finder"
        @click="$emit('revealInFinder')"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 8.93 2H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2z" />
        </svg>
      </button>
    </div>
  </div>
</template>
