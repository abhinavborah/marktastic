import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Persistent cross-compile cache: "pdfHash:pageNum:zoom" → dataUrl
const persistentCache = new Map<string, string>();

function getCacheKey(pdfHash: string, pageNum: number, zoom: number): string {
  return `${pdfHash}:${pageNum}:${zoom}`;
}

async function hashBytes(bytes: Uint8Array): Promise<string> {
  const hashBuffer = await crypto.subtle.digest("SHA-256", bytes);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map((b) => b.toString(16).padStart(2, "0")).join("").slice(0, 16);
}

const pages = ref<(string | null)[]>([]);
const stalePages = ref<(string | null)[]>([]);
const totalPages = ref(0);
const rendering = ref(false);
const renderError = ref<string | null>(null);
const isRecompiling = ref(false);
const currentPdfHash = ref("");

export function usePdfRenderer(
  pdfBytesRef: Ref<Uint8Array | null>,
  visiblePageNumbersRef: Ref<Set<number>>
) {
  async function renderForPdfBytes(bytes: Uint8Array | null) {
    if (!bytes || bytes.length === 0) {
      pages.value = [];
      stalePages.value = [];
      totalPages.value = 0;
      isRecompiling.value = false;
      rendering.value = false;
      return;
    }

    const newPdfHash = await hashBytes(bytes);
    const newZoom = 2.0;
    currentPdfHash.value = newPdfHash;

    // Save current display as stale for smooth stale-while-revalidate transition
    stalePages.value = [...pages.value];
    isRecompiling.value = true;
    rendering.value = true;
    renderError.value = null;

    // Get page count
    let count: number;
    try {
      count = await invoke<number>("get_pdf_page_count", {
        pdfBytes: Array.from(bytes),
      });
    } catch (err: any) {
      renderError.value = String(err);
      isRecompiling.value = false;
      rendering.value = false;
      return;
    }

    totalPages.value = count;

    // Build display: cache hit → cached image, cache miss → stale page fallback
    const merged: (string | null)[] = [];
    const toRender: number[] = [];

    for (let i = 0; i < count; i++) {
      const key = getCacheKey(newPdfHash, i, newZoom);
      const cached = persistentCache.get(key);
      if (cached) {
        merged[i] = cached;
      } else if (i < stalePages.value.length && stalePages.value[i]) {
        merged[i] = stalePages.value[i]; // show stale while re-rendering
        toRender.push(i);
      } else {
        merged[i] = null;
        toRender.push(i);
      }
    }

    pages.value = merged;

    if (toRender.length === 0) {
      rendering.value = false;
      isRecompiling.value = false;
      return;
    }

    // Render visible pages first (priority)
    const visible = visiblePageNumbersRef.value;
    const visibleToRender = toRender.filter((p) => visible.has(p));
    const hiddenToRender = toRender.filter((p) => !visible.has(p));

    if (visibleToRender.length > 0) {
      try {
        const dpr = window.devicePixelRatio || 1;
        const result = await invoke<Array<[number, string]>>("render_pdf_page_range", {
          pdfBytes: Array.from(bytes),
          pageNumbers: visibleToRender,
          zoom: newZoom,
          devicePixelRatio: dpr,
        });
        const updated = [...pages.value];
        for (const [pageNum, dataUrl] of result) {
          updated[pageNum] = dataUrl;
          persistentCache.set(getCacheKey(newPdfHash, pageNum, newZoom), dataUrl);
        }
        pages.value = updated;
      } catch (err: any) {
        renderError.value = String(err);
      }
    }

    // Render hidden pages in background
    if (hiddenToRender.length > 0) {
      try {
        const dpr = window.devicePixelRatio || 1;
        const result = await invoke<Array<[number, string]>>("render_pdf_page_range", {
          pdfBytes: Array.from(bytes),
          pageNumbers: hiddenToRender,
          zoom: newZoom,
          devicePixelRatio: dpr,
        });
        const updated = [...pages.value];
        for (const [pageNum, dataUrl] of result) {
          updated[pageNum] = dataUrl;
          persistentCache.set(getCacheKey(newPdfHash, pageNum, newZoom), dataUrl);
        }
        pages.value = updated;
      } catch {
        // silent for hidden pages
      }
    }

    rendering.value = false;
    isRecompiling.value = false;
  }

  async function renderOnScroll() {
    const bytes = pdfBytesRef.value;
    if (!bytes || bytes.length === 0) return;
    if (!currentPdfHash.value) return;

    const hash = currentPdfHash.value;
    const zoom = 2.0;
    const visible = visiblePageNumbersRef.value;

    const toRender: number[] = [];
    for (const pageNum of visible) {
      if (pageNum >= totalPages.value) continue;
      const key = getCacheKey(hash, pageNum, zoom);
      if (!persistentCache.has(key)) {
        toRender.push(pageNum);
      }
    }

    if (toRender.length === 0) return;

    rendering.value = true;
    try {
      const dpr = window.devicePixelRatio || 1;
      const result = await invoke<Array<[number, string]>>("render_pdf_page_range", {
        pdfBytes: Array.from(bytes),
        pageNumbers: toRender,
        zoom,
        devicePixelRatio: dpr,
      });
      const updated = [...pages.value];
      for (const [pageNum, dataUrl] of result) {
        updated[pageNum] = dataUrl;
        persistentCache.set(getCacheKey(hash, pageNum, zoom), dataUrl);
      }
      pages.value = updated;
    } catch (err: any) {
      renderError.value = String(err);
    } finally {
      rendering.value = false;
    }
  }

  // When pdfBytes changes: full recompile with stale-while-revalidate
  watch(
    () => pdfBytesRef.value,
    async (bytes) => {
      await renderForPdfBytes(bytes);
    },
    { immediate: false }
  );

  // When visible pages change (scroll): render any uncached visible pages
  watch(
    () => Array.from(visiblePageNumbersRef.value).sort().join(","),
    async () => {
      if (isRecompiling.value) return; // renderForPdfBytes already handles visible pages
      await renderOnScroll();
    }
  );

  return {
    pages,
    totalPages,
    rendering,
    renderError,
    isRecompiling,
  };
}
