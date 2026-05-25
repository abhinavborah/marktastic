import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function usePdfRenderer(
  pdfBytesRef: Ref<Uint8Array | null>,
  zoomRef: Ref<number>
) {
  const pages = ref<string[]>([]);
  const rendering = ref(false);
  const renderError = ref<string | null>(null);

  watch(
    [pdfBytesRef, zoomRef],
    async () => {
      if (!pdfBytesRef.value || pdfBytesRef.value.length === 0) {
        pages.value = [];
        renderError.value = null;
        return;
      }

      rendering.value = true;
      renderError.value = null;

      try {
        const dpr = window.devicePixelRatio || 1;
        const result = await invoke<string[]>("render_pdf_pages", {
          pdfBytes: Array.from(pdfBytesRef.value),
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
    },
    { immediate: false }
  );

  return {
    pages,
    rendering,
    renderError,
  };
}
