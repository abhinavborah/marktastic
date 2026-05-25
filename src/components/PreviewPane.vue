<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  pdfUrl: string | null;
  loading: boolean;
  error: string | null;
}>();

const hasPdf = computed(() => !!props.pdfUrl);
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Loading state -->
    <div
      v-if="loading"
      class="flex-1 flex items-center justify-center text-muted-foreground"
    >
      <div class="flex items-center gap-2">
        <svg
          class="w-5 h-5 animate-spin"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
        </svg>
        Rendering PDF...
      </div>
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
      class="flex-1 flex items-center justify-center text-muted-foreground"
    >
      <div class="text-center space-y-2">
        <svg
          class="w-12 h-12 mx-auto opacity-50"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
        >
          <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
          <polyline points="14 2 14 8 20 8" />
        </svg>
        <p>Start typing to see the preview</p>
      </div>
    </div>

    <!-- PDF iframe -->
    <iframe
      v-else
      :src="pdfUrl ?? undefined"
      class="flex-1 w-full border-0"
      title="PDF Preview"
    />
  </div>
</template>
