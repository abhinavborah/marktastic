<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick, onUnmounted } from "vue";

const props = defineProps<{
  pages: (string | null)[];
  totalPages: number;
  rendering: boolean;
  error: string | null;
  zoom: number;
}>();

const emit = defineEmits<{
  (e: "update:visiblePages", pages: Set<number>): void;
}>();

const scrollerRef = ref<HTMLDivElement | null>(null);

// Use a Map for stable ref tracking across re-renders
const pageElements = new Map<number, HTMLElement>();

function registerPageEl(el: HTMLElement | null, idx: number) {
  if (el) {
    pageElements.set(idx, el);
  } else {
    pageElements.delete(idx);
  }
}

let observer: IntersectionObserver | null = null;

function setupObserver() {
  if (observer) {
    observer.disconnect();
    observer = null;
  }
  if (!scrollerRef.value) return;

  console.log("[PreviewPane] setupObserver, totalPages:", props.totalPages, "elements:", pageElements.size);

  observer = new IntersectionObserver(
    (entries) => {
      const visible = new Set<number>();
      for (const entry of entries) {
        const target = entry.target as HTMLElement;
        const idx = Number(target.dataset.pageIdx);
        console.log("[PreviewPane] IO entry idx=", idx, "intersecting=", entry.isIntersecting, "ratio=", entry.intersectionRatio);
        if (entry.isIntersecting) {
          visible.add(idx);
        }
      }

      if (visible.size === 0) {
        console.log("[PreviewPane] IO: no intersecting entries");
        return;
      }

      // Buffer: visible page ± 1 neighbor
      const buffered = new Set(visible);
      for (const idx of visible) {
        if (idx > 0) buffered.add(idx - 1);
        if (idx < props.totalPages - 1) buffered.add(idx + 1);
      }

      console.log("[PreviewPane] emitting visiblePages:", Array.from(buffered).sort((a, b) => a - b));
      emit("update:visiblePages", buffered);
    },
    {
      root: scrollerRef.value,
      rootMargin: "200px 0px",
      threshold: 0,
    }
  );

  for (const [, el] of pageElements) {
    if (el && el.isConnected) {
      observer.observe(el);
    }
  }
}

function teardownObserver() {
  if (observer) {
    observer.disconnect();
    observer = null;
  }
  pageElements.clear();
}

onMounted(() => {
  console.log("[PreviewPane] mounted, totalPages:", props.totalPages);
  nextTick(() => {
    setupObserver();
  });
});

onUnmounted(() => {
  teardownObserver();
});

watch(() => props.totalPages, (newVal) => {
  console.log("[PreviewPane] totalPages changed:", newVal);
  teardownObserver();
  nextTick(() => {
    setupObserver();
  });
});

const hasPages = computed(() => props.totalPages > 0);
const displayScale = computed(() => props.zoom / 2.0);
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

    <!-- Pages with lazy loading -->
    <div v-else ref="scrollerRef" class="flex-1 overflow-auto relative">
      <!-- Subtle re-rendering indicator for additional pages -->
      <div
        v-if="rendering"
        class="absolute top-2 right-2 z-20 flex items-center gap-2 bg-card/90 backdrop-blur-sm border rounded-full px-3 py-1.5 text-xs text-muted-foreground shadow-sm"
      >
        <svg class="w-3 h-3 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
        </svg>
        <span>Loading pages...</span>
      </div>

      <div
        v-for="i in totalPages"
        :key="i"
        :ref="(el: any) => registerPageEl(el, i - 1)"
        :data-page-idx="i - 1"
        class="flex justify-center p-4"
      >
        <!-- Rendered page -->
        <img
          v-if="pages[i - 1]"
          :src="pages[i - 1]!"
          :alt="`Page ${i}`"
          class="shadow-lg"
          :style="{
            zoom: displayScale,
            backgroundColor: 'white',
          }"
        />

        <!-- Placeholder for unrendered page -->
        <div
          v-else
          class="w-full max-w-[800px] h-[400px] bg-muted/30 rounded-lg border border-dashed border-muted-foreground/20 flex flex-col items-center justify-center gap-2"
        >
          <svg class="w-6 h-6 text-muted-foreground/50" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
            <polyline points="14 2 14 8 20 8" />
          </svg>
          <span class="text-xs text-muted-foreground/60">Page {{ i }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
