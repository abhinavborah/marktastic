<script setup lang="ts">
import { useToast } from "../composables/useToast";

const { toasts, removeToast } = useToast();

function typeClasses(type: string) {
  switch (type) {
    case "success":
      return "bg-emerald-600 text-white border-emerald-500";
    case "error":
      return "bg-red-600 text-white border-red-500";
    case "warning":
      return "bg-amber-500 text-white border-amber-400";
    case "info":
    default:
      return "bg-slate-700 text-white border-slate-600";
  }
}

function iconFor(type: string) {
  switch (type) {
    case "success":
      return "✓";
    case "error":
      return "✕";
    case "warning":
      return "!";
    case "info":
    default:
      return "ℹ";
  }
}
</script>

<template>
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="pointer-events-auto flex items-center gap-2.5 px-4 py-2.5 rounded-lg border shadow-lg text-sm font-medium min-w-[16rem] max-w-[24rem]"
        :class="typeClasses(toast.type)"
        role="alert"
      >
        <span class="flex items-center justify-center w-5 h-5 rounded-full bg-white/20 text-xs font-bold shrink-0">
          {{ iconFor(toast.type) }}
        </span>
        <span class="flex-1">{{ toast.message }}</span>
        <button
          class="opacity-70 hover:opacity-100 transition-opacity"
          @click="removeToast(toast.id)"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12"/></svg>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.25s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(1rem);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(1rem);
}
</style>
