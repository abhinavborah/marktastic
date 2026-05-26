<script setup lang="ts">
import { ref, computed } from "vue";

const props = defineProps<{
  pages: string[];
  totalPages: number;
  rendering: boolean;
  error: string | null;
  zoom: number;
}>();

const scrollerRef = ref<HTMLDivElement | null>(null);

const hasPages = computed(() => props.totalPages > 0);

// SVG is vector — zoom is pure CSS, no re-render needed
const displayScale = computed(() => props.zoom / 2.0);

function svgToDataUrl(svg: string): string {
  if (svg.startsWith("data:")) return svg;
  const encoded = svg
    .replace(/%/g, "%25")
    .replace(/#/g, "%23")
    .replace(/\n/g, "%0A");
  return `data:image/svg+xml,${encoded}`;
}
</script>

<template>
  <div class="h-full flex flex-col relative">
    <!-- Loading state (initial) -->
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

    <!-- SVG Pages via <img> — off-thread parsing -->
    <div v-else ref="scrollerRef" class="flex-1 overflow-auto">
      <div
        v-for="(page, i) in pages"
        :key="i"
        class="flex justify-center p-4"
      >
        <img
          :src="svgToDataUrl(page)"
          :alt="`Page ${i + 1}`"
          class="shadow-lg"
          :style="{
            zoom: displayScale,
            backgroundColor: 'white',
          }"
        />
      </div>
    </div>
  </div>
</template>
