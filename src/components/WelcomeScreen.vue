<script setup lang="ts">
defineEmits<{
  (e: "openFile"): void;
  (e: "openFolder"): void;
}>();
</script>

<template>
  <div class="flex flex-col items-center justify-center h-full gap-8 p-8">
    <div class="text-center space-y-4">
      <h1 class="text-5xl font-bold tracking-tight">Marktastic</h1>
      <p class="text-xl text-muted-foreground">
        Convert Markdown to beautiful PDFs
      </p>
    </div>

    <div class="flex gap-4">
      <button
        class="inline-flex items-center gap-2 px-6 py-3 text-lg font-medium rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors shadow-sm"
        @click="$emit('openFile')"
      >
        <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg>
        Open File
      </button>
      <button
        class="inline-flex items-center gap-2 px-6 py-3 text-lg font-medium rounded-lg bg-secondary text-secondary-foreground hover:bg-secondary/90 transition-colors shadow-sm"
        @click="$emit('openFolder')"
      >
        <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
        Open Folder
      </button>
    </div>
  </div>
</template>
