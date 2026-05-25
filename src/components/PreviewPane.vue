<script setup lang="ts">
import { ref, watch, computed } from "vue";

const props = defineProps<{
  pages: string[];
  rendering: boolean;
  error: string | null;
  zoom: number;
}>();

const hasPages = computed(() => props.pages.length > 0);

// Track which zoom level the current images were rendered at
const renderedZoom = ref(props.zoom);

// When new pages arrive, they were rendered at current zoom
watch(
  () => props.pages,
  () => {
    renderedZoom.value = props.zoom;
  },
  { deep: true }
);

// CSS scale = target zoom / rendered zoom
// When zoom changes: cssScale > 1 or < 1 → immediate visual feedback
// When new pages arrive: renderedZoom catches up → cssScale snaps to 1
const cssScale = computed(() => {
  if (renderedZoom.value === 0 || renderedZoom.value === props.zoom) {
    return 1;
  }
  return props.zoom / renderedZoom.value;
});

const isScaling = computed(() => Math.abs(cssScale.value - 1) > 0.01);
</script>

<template>
  <div class="h-full flex flex-col relative">
    <!-- Loading state (initial, no pages) -->
    <div
      v-if="rendering && !hasPages"
      class="flex-1 flex flex-col items-center justify-center text-muted-foreground gap-3"
    >
      <svg class="w-8 h-8 animate-spin text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
      </svg>
      <p class="text-sm font-medium">Rendering PDF...</p>
    </div>

    <!-- Error state -->
    <div v-else-if="error" class="flex-1 flex items-center justify-center text-destructive p-4 text-center">
      <div class="space-y-2">
        <p class="font-medium">Failed to render PDF</p>
        <p class="text-sm text-muted-foreground">{{ error }}</p>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else-if="!hasPages" class="flex-1 flex flex-col items-center justify-center text-muted-foreground gap-3">
      <svg class="w-16 h-16 opacity-40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
        <polyline points="14 2 14 8 20 8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <line x1="10" y1="9" x2="8" y2="9" />
      </svg>
      <p class="text-sm">Start typing to see the preview</p>
    </div>

    <!-- Pages with CSS zoom transition -->
    <div v-else class="flex-1 overflow-auto relative">
      <!-- Subtle re-rendering indicator -->
      <div
        v-if="rendering"
        class="absolute top-2 right-2 z-20 flex items-center gap-2 bg-card/90 backdrop-blur-sm border rounded-full px-3 py-1.5 text-xs text-muted-foreground shadow-sm"
      >
        <svg class="w-3 h-3 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
        </svg>
        <span>Updating...</span>
      </div>

      <div
        v-for="(page, i) in pages"
        :key="i"
        class="flex justify-center p-4"
      >
        <img
          :src="page"
          :alt="`Page ${i + 1}`"
          class="shadow-lg max-w-full"
          :style="{
            transform: `scale(${cssScale})`,
            transformOrigin: 'top center',
            transition: isScaling ? 'transform 0.15s ease-out' : 'none',
            backgroundColor: 'white',
          }"
        />
      </div>
    </div>
  </div>
</template>
