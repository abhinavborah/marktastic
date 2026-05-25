<script setup lang="ts">
import { computed, ref, onMounted, watch } from "vue";

const props = defineProps<{
  pdfUrl: string | null;
  loading: boolean;
  error: string | null;
  zoom?: number;
}>();

const emit = defineEmits<{
  (e: "iframeReady", el: HTMLIFrameElement): void;
}>();

const hasPdf = computed(() => !!props.pdfUrl);
const iframeRef = ref<HTMLIFrameElement | null>(null);
const iframeLoaded = ref(false);

onMounted(() => {
  if (iframeRef.value) {
    emit("iframeReady", iframeRef.value);
  }
});

function onIframeLoad() {
  iframeLoaded.value = true;
  if (iframeRef.value) {
    emit("iframeReady", iframeRef.value);
  }
}

// Reset loaded state when pdfUrl changes
watch(() => props.pdfUrl, () => {
  iframeLoaded.value = false;
});
</script>

<template>
  <div class="h-full flex flex-col relative">
    <!-- Loading state -->
    <div
      v-if="loading"
      class="flex-1 flex flex-col items-center justify-center text-muted-foreground gap-3"
    >
      <svg
        class="w-8 h-8 animate-spin text-primary"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
      </svg>
      <p class="text-sm font-medium">Compiling PDF...</p>
    </div>

    <!-- Error state -->
    <div
      v-else-if="error"
      class="flex-1 flex items-center justify-center text-destructive p-4 text-center"
    >
      <div class="space-y-2">
        <p class="font-medium">Failed to render PDF</p>
        <p class="text-sm text-muted-foreground">{{ error }}</p>
      </div>
    </div>

    <!-- Empty state -->
    <div
      v-else-if="!hasPdf"
      class="flex-1 flex flex-col items-center justify-center text-muted-foreground gap-3"
    >
      <svg
        class="w-16 h-16 opacity-40"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1"
      >
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
        <polyline points="14 2 14 8 20 8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <line x1="10" y1="9" x2="8" y2="9" />
      </svg>
      <p class="text-sm">Start typing to see the preview</p>
    </div>

    <!-- PDF iframe with dark overlay to prevent white flash -->
    <template v-else>
      <!-- Dark overlay shown while iframe is loading its content -->
      <div
        v-if="!iframeLoaded"
        class="absolute inset-0 z-10 bg-black flex items-center justify-center"
      >
        <div class="flex flex-col items-center gap-2 text-muted-foreground">
          <svg
            class="w-6 h-6 animate-spin text-primary"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
          </svg>
          <span class="text-sm">Loading PDF...</span>
        </div>
      </div>
      <div
        class="flex-1 w-full overflow-auto"
        :style="{
          transform: `scale(${props.zoom ?? 1})`,
          transformOrigin: 'top left',
          width: `${100 / (props.zoom ?? 1)}%`,
          height: `${100 / (props.zoom ?? 1)}%`,
        }"
      >
        <iframe
          ref="iframeRef"
          :src="pdfUrl ?? undefined"
          class="w-full h-full border-0 bg-white"
          title="PDF Preview"
          @load="onIframeLoad"
        />
      </div>
    </template>
  </div>
</template>
