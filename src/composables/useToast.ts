import { ref } from "vue";

export type ToastType = "info" | "success" | "warning" | "error" | "loading";

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
  duration: number;
}

let nextId = 0;
const toasts = ref<Toast[]>([]);
const timers = new Map<number, ReturnType<typeof setTimeout>>();

function removeToast(id: number) {
  const timer = timers.get(id);
  if (timer) {
    clearTimeout(timer);
    timers.delete(id);
  }
  toasts.value = toasts.value.filter((t) => t.id !== id);
}

export function useToast() {
  function showToast(message: string, type: ToastType = "info", duration = 4000): number {
    const id = ++nextId;
    const toast: Toast = { id, message, type, duration };
    toasts.value.push(toast);

    if (duration > 0) {
      const timer = setTimeout(() => {
        removeToast(id);
      }, duration);
      timers.set(id, timer);
    }

    return id;
  }

  function dismiss(id: number) {
    removeToast(id);
  }

  function info(message: string, duration?: number) {
    return showToast(message, "info", duration);
  }
  function success(message: string, duration?: number) {
    return showToast(message, "success", duration);
  }
  function warning(message: string, duration?: number) {
    return showToast(message, "warning", duration);
  }
  function error(message: string, duration?: number) {
    return showToast(message, "error", duration);
  }
  function loading(message: string, duration?: number) {
    return showToast(message, "loading", duration);
  }

  return {
    toasts,
    showToast,
    info,
    success,
    warning,
    error,
    loading,
    removeToast,
    dismiss,
  };
}
