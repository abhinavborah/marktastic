import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pages = ref<string[]>([]);
const rendering = ref(false);
const renderError = ref<string | null>(null);

export function usePdfRenderer(pdfBytesRef: Ref<Uint8Array | null>) {
  watch(
    () => pdfBytesRef.value,
    async (bytes) => {
      if (!bytes || bytes.length === 0) {
        pages.value = [];
        return;
      }

      rendering.value = true;
      renderError.value = null;

      try {
        const dpr = window.devicePixelRatio || 1;
        // FIXED: Always render at 2.0× zoom regardless of display zoom
        const result = await invoke<string[]>("render_pdf_pages", {
          pdfBytes: Array.from(bytes),
          zoom: 2.0,
          devicePixelRatio: dpr,
        });
        pages.value = result;
      } catch (err: any) {
        renderError.value = String(err);
        console.error("PDF rendering failed:", err);
      } finally {
        rendering.value = false;
      }
    },
    { immediate: false }
  );

  return {
    pages,
    rendering,
    renderError,
  };
}
