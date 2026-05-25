<script setup lang="ts">
import { ref, computed } from "vue";

type PaneMode = "both" | "editor" | "preview";

const props = defineProps<{
  modelValue: PaneMode;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", mode: PaneMode): void;
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

function startDrag(e: MouseEvent) {
  if (!isBoth.value) return;
  isDragging.value = true;
  document.addEventListener("mousemove", onDrag);
  document.addEventListener("mouseup", stopDrag);
  e.preventDefault();
}

function onDrag(e: MouseEvent) {
  if (!isDragging.value) return;
  const container = document.getElementById("split-container");
  if (!container) return;
  const rect = container.getBoundingClientRect();
  const pct = ((e.clientX - rect.left) / rect.width) * 100;
  editorWidth.value = Math.max(20, Math.min(80, pct));
}

function stopDrag() {
  isDragging.value = false;
  document.removeEventListener("mousemove", onDrag);
  document.removeEventListener("mouseup", stopDrag);
}

function togglePane(mode: PaneMode) {
  paneMode.value = mode;
}
</script>

<template>
  <div id="split-container" class="flex h-full relative">
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

    <!-- Pane toggle buttons (floating) -->
    <div
      class="absolute top-2 right-2 flex gap-1 bg-card/90 backdrop-blur-sm border rounded-md p-1 shadow-sm z-20"
    >
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
    </div>
  </div>
</template>
