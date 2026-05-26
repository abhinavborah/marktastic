<script setup lang="ts">
import { useToast } from "../composables/useToast";

const { toasts, removeToast } = useToast();

function typeClasses(type: string) {
  switch (type) {
    case "success":
      return "text-green-600 dark:text-green-400";
    case "error":
      return "text-red-600 dark:text-red-400";
    case "warning":
      return "text-amber-600 dark:text-amber-400";
    case "info":
    case "loading":
    default:
      return "text-muted-foreground";
  }
}

function isSpinning(type: string) {
  return type === "info" || type === "loading";
}
</script>

<template>
  <div class="flex flex-col items-center gap-2 pointer-events-none w-56">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="w-full pointer-events-auto flex items-center gap-2 bg-card/90 backdrop-blur-sm border rounded-full px-3 py-1.5 text-xs shadow-sm"
        :class="typeClasses(toast.type)"
        role="alert"
      >
        <!-- Spinner for info/loading -->
        <svg
          v-if="isSpinning(toast.type)"
          class="w-3 h-3 animate-spin shrink-0"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
        </svg>

        <!-- Static icon for success -->
        <svg
          v-else-if="toast.type === 'success'"
          class="w-3 h-3 shrink-0"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M20 6L9 17l-5-5" />
        </svg>

        <!-- Static icon for warning -->
        <svg
          v-else-if="toast.type === 'warning'"
          class="w-3 h-3 shrink-0"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
          <line x1="12" y1="9" x2="12" y2="13" />
          <line x1="12" y1="17" x2="12.01" y2="17" />
        </svg>

        <!-- Static icon for error -->
        <svg
          v-else-if="toast.type === 'error'"
          class="w-3 h-3 shrink-0"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="10" />
          <line x1="15" y1="9" x2="9" y2="15" />
          <line x1="9" y1="9" x2="15" y2="15" />
        </svg>

        <span>{{ toast.message }}</span>

        <button
          class="ml-0.5 p-0.5 rounded-full hover:bg-muted/50 transition-colors shrink-0"
          @click="removeToast(toast.id)"
        >
          <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
