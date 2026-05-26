import { ref, watch, type Ref } from "vue";
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
  async function renderMissingPages() {
    if (!pdfBytesRef.value || pdfBytesRef.value.length === 0) return;

    const visible = visiblePageNumbersRef.value;
    if (visible.size === 0) {
      console.log("[usePdfRenderer] renderMissingPages: visible set is empty, skipping");
      return;
    }

    const missingPages: number[] = [];
    for (const pageNum of visible) {
      if (!pageCache.has(pageNum)) {
        missingPages.push(pageNum);
      }
    }

    console.log("[usePdfRenderer] renderMissingPages: visible=", Array.from(visible), "missing=", missingPages);

    if (missingPages.length === 0) {
      console.log("[usePdfRenderer] renderMissingPages: all visible pages already cached");
      return;
    }

    rendering.value = true;
    renderError.value = null;

    try {
      const dpr = window.devicePixelRatio || 1;
      console.log("[usePdfRenderer] invoking render_pdf_page_range for pages:", missingPages);
      const result = await invoke<Array<[number, string]>>("render_pdf_page_range", {
        pdfBytes: Array.from(pdfBytesRef.value),
        pageNumbers: missingPages,
        zoom: 2.0,
        devicePixelRatio: dpr,
      });

      console.log("[usePdfRenderer] received", result.length, "rendered pages");

      for (const [pageNum, dataUrl] of result) {
        pageCache.set(pageNum, dataUrl);
      }

      // Update the reactive pages array directly so Vue detects the change
      const newPages = [...pages.value];
      for (let i = 0; i < totalPages.value; i++) {
        const cached = pageCache.get(i);
        if (cached !== undefined) {
          newPages[i] = cached;
        }
      }
      pages.value = newPages;
      console.log("[usePdfRenderer] pages.value updated, cached count:", pageCache.size, "/", totalPages.value);
    } catch (err: any) {
      renderError.value = String(err);
      console.error("[usePdfRenderer] PDF page rendering failed:", err);
    } finally {
      rendering.value = false;
    }
  }

  // When pdfBytes changes: get page count, clear cache, render first visible pages
  watch(
    () => pdfBytesRef.value,
    async (bytes) => {
      console.log("[usePdfRenderer] pdfBytes changed, bytes length:", bytes?.length ?? 0);
      pageCache.clear();
      pages.value = [];
      totalPages.value = 0;
      renderError.value = null;

      if (!bytes || bytes.length === 0) return;

      try {
        const count = await invoke<number>("get_pdf_page_count", {
          pdfBytes: Array.from(bytes),
        });
        console.log("[usePdfRenderer] PDF page count:", count);
        totalPages.value = count;

        // Initialize pages array with nulls
        pages.value = new Array(count).fill(null);

        // Default to first 3 pages visible
        const initial = new Set<number>();
        for (let i = 0; i < Math.min(3, count); i++) {
          initial.add(i);
        }
        visiblePageNumbersRef.value = initial;
        console.log("[usePdfRenderer] set initial visible pages:", Array.from(initial));

        await renderMissingPages();
      } catch (err: any) {
        renderError.value = String(err);
        console.error("[usePdfRenderer] Failed to get PDF page count:", err);
      }
    },
    { immediate: false }
  );

  // When visible pages change: render missing ones
  watch(
    visiblePageNumbersRef,
    (newVal, oldVal) => {
      const newStr = Array.from(newVal).sort().join(",");
      const oldStr = Array.from(oldVal).sort().join(",");
      if (newStr === oldStr) return;
      console.log("[usePdfRenderer] visiblePageNumbers changed:", oldStr, "→", newStr);
      renderMissingPages();
    },
    { deep: true }
  );

  return {
    pages,
    totalPages,
    rendering,
    renderError,
  };
}
