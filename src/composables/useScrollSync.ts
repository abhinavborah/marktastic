import { watch, onBeforeUnmount } from "vue";

/**
 * One-directional scroll sync: CodeMirror editor scroll → PDF preview iframe scroll.
 * Uses percentage-based sync: scroll position as a fraction of total scrollable area.
 */
export function useScrollSync(
  editorViewRef: any,
  previewIframeRef: any,
  debounceMs = 50
) {
  let scrollTimeout: ReturnType<typeof setTimeout> | null = null;

  function getEditorScrollPct(view: any): number {
    if (!view || !view.scrollDOM) return 0;
    const dom = view.scrollDOM;
    const max = Math.max(1, dom.scrollHeight - dom.clientHeight);
    return dom.scrollTop / max;
  }

  function setIframeScrollPct(iframe: HTMLIFrameElement, pct: number) {
    try {
      const doc = iframe.contentDocument || iframe.contentWindow?.document;
      if (!doc) return;
      const html = doc.documentElement;
      const body = doc.body;
      const maxScroll = Math.max(
        1,
        body.scrollHeight - html.clientHeight,
        html.scrollHeight - html.clientHeight
      );
      const target = pct * maxScroll;
      html.scrollTop = target;
      body.scrollTop = target;
    } catch {
      // Silently ignore cross-origin or iframe not ready
    }
  }

  function onEditorScroll() {
    const view = editorViewRef.value;
    const iframe = previewIframeRef.value;
    if (!view || !iframe) return;

    if (scrollTimeout) clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
      const pct = getEditorScrollPct(view);
      setIframeScrollPct(iframe, pct);
    }, debounceMs);
  }

  // Watch for editor view becoming available
  const unwatch = watch(
    () => editorViewRef.value,
    (view) => {
      if (view && view.scrollDOM) {
        view.scrollDOM.addEventListener("scroll", onEditorScroll);
      }
    },
    { immediate: true }
  );

  onBeforeUnmount(() => {
    unwatch();
    const view = editorViewRef.value;
    if (view && view.scrollDOM) {
      view.scrollDOM.removeEventListener("scroll", onEditorScroll);
    }
    if (scrollTimeout) clearTimeout(scrollTimeout);
  });
}
