import { ref, watch, type Ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Cache: page number → data URL (persists across renders, module-scoped)
const pageCache = new Map<number, string>();

const pages = ref<(string | null)[]>([]);
const totalPages = ref(0);
const rendering = ref(false);
const renderError = ref<string | null>(null);

export function usePdfRenderer(
  pdfBytesRef: Ref<Uint8Array | null>,
  visiblePageNumbersRef: Ref<Set<number>>
) {
  const cachedPages = computed(() => {
    const result: (string | null)[] = [];
    for (let i = 0; i < totalPages.value; i++) {
      result.push(pageCache.get(i) ?? null);
    }
    return result;
  });

  async function renderMissingPages() {
    if (!pdfBytesRef.value || pdfBytesRef.value.length === 0) return;

    const visible = visiblePageNumbersRef.value;
    if (visible.size === 0) return;

    const missingPages: number[] = [];
    for (const pageNum of visible) {
      if (!pageCache.has(pageNum)) {
        missingPages.push(pageNum);
      }
    }

    if (missingPages.length === 0) return;

    rendering.value = true;
    renderError.value = null;

    try {
      const dpr = window.devicePixelRatio || 1;
      const result = await invoke<Array<[number, string]>>("render_pdf_page_range", {
        pdfBytes: Array.from(pdfBytesRef.value),
        pageNumbers: missingPages,
        zoom: 2.0,
        devicePixelRatio: dpr,
      });

      for (const [pageNum, dataUrl] of result) {
        pageCache.set(pageNum, dataUrl);
      }

      // Rebuild pages array from cache
      const newPages: (string | null)[] = [];
      for (let i = 0; i < totalPages.value; i++) {
        newPages.push(pageCache.get(i) ?? null);
      }
      pages.value = newPages;
    } catch (err: any) {
      renderError.value = String(err);
      console.error("PDF page rendering failed:", err);
    } finally {
      rendering.value = false;
    }
  }

  // When pdfBytes changes: get page count, clear cache, render first visible pages
  watch(
    () => pdfBytesRef.value,
    async (bytes) => {
      pageCache.clear();
      pages.value = [];
      totalPages.value = 0;
      renderError.value = null;

      if (!bytes || bytes.length === 0) return;

      try {
        const count = await invoke<number>("get_pdf_page_count", {
          pdfBytes: Array.from(bytes),
        });
        totalPages.value = count;

        // Default to first 3 pages visible
        const initial = new Set<number>();
        for (let i = 0; i < Math.min(3, count); i++) {
          initial.add(i);
        }
        visiblePageNumbersRef.value = initial;

        await renderMissingPages();
      } catch (err: any) {
        renderError.value = String(err);
        console.error("Failed to get PDF page count:", err);
      }
    },
    { immediate: false }
  );

  // When visible pages change: render missing ones
  watch(
    () => new Set(visiblePageNumbersRef.value),
    () => {
      renderMissingPages();
    },
    { deep: true }
  );

  return {
    pages: cachedPages,
    totalPages,
    rendering,
    renderError,
  };
}
