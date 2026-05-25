import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pages = ref<string[]>([]);
const rendering = ref(false);
const renderError = ref<string | null>(null);
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

export function usePdfRenderer(
  pdfBytesRef: Ref<Uint8Array | null>,
  zoomRef: Ref<number>
) {
  watch(
    [pdfBytesRef, zoomRef],
    () => {
      if (!pdfBytesRef.value || pdfBytesRef.value.length === 0) {
        pages.value = [];
        renderError.value = null;
        return;
      }

      if (debounceTimer) clearTimeout(debounceTimer);

      debounceTimer = setTimeout(async () => {
        rendering.value = true;
        renderError.value = null;

        try {
          const dpr = window.devicePixelRatio || 1;
          // Safe cast: null was checked before setTimeout
          const bytes = pdfBytesRef.value as Uint8Array;
          const result = await invoke<string[]>("render_pdf_pages", {
            pdfBytes: Array.from(bytes),
            zoom: zoomRef.value,
            devicePixelRatio: dpr,
          });
          pages.value = result;
        } catch (err: any) {
          renderError.value = String(err);
          console.error("PDF rendering failed:", err);
        } finally {
          rendering.value = false;
        }
      }, 200);
    },
    { immediate: false }
  );

  return {
    pages,
    rendering,
    renderError,
  };
}
