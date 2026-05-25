import { ref } from "vue";

export type ToastType = "info" | "success" | "warning" | "error";

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
  duration: number;
}

let nextId = 0;
const toasts = ref<Toast[]>([]);

function removeToast(id: number) {
  toasts.value = toasts.value.filter((t) => t.id !== id);
}

export function useToast() {
  function showToast(message: string, type: ToastType = "info", duration = 4000) {
    const id = ++nextId;
    const toast: Toast = { id, message, type, duration };
    toasts.value.push(toast);

    setTimeout(() => {
      removeToast(id);
    }, duration);
  }

  function info(message: string, duration?: number) {
    showToast(message, "info", duration);
  }
  function success(message: string, duration?: number) {
    showToast(message, "success", duration);
  }
  function warning(message: string, duration?: number) {
    showToast(message, "warning", duration);
  }
  function error(message: string, duration?: number) {
    showToast(message, "error", duration);
  }

  return {
    toasts,
    showToast,
    info,
    success,
    warning,
    error,
    removeToast,
  };
}
