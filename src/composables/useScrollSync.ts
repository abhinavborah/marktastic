import { ref, watch, onBeforeUnmount } from "vue";

/**
 * Bidirectional scroll sync between CodeMirror editor and PDF preview iframe.
 * Uses percentage-based sync: scroll position as a fraction of total scrollable area.
 * A sync flag prevents infinite loops.
 */
export function useScrollSync(
  editorViewRef: any,
  previewIframeRef: any,
  debounceMs = 50
) {
  const isSyncing = ref(false);
  let editorScrollPct = ref(0);
  let iframeScrollPct = ref(0);

  let editorListener: (() => void) | null = null;
  let iframeListener: (() => void) | null = null;

  function getEditorScrollInfo(view: any) {
    if (!view) return { top: 0, maxScroll: 1, clientHeight: 1 };
    const scrollDOM = view.scrollDOM;
    const top = scrollDOM.scrollTop;
    const scrollHeight = scrollDOM.scrollHeight;
    const clientHeight = scrollDOM.clientHeight;
    const maxScroll = Math.max(1, scrollHeight - clientHeight);
    return { top, maxScroll, clientHeight };
  }

  function getIframeScrollInfo(iframe: HTMLIFrameElement) {
    try {
      const doc = iframe.contentDocument || iframe.contentWindow?.document;
      if (!doc) return { top: 0, maxScroll: 1 };
      const body = doc.body;
      const html = doc.documentElement;
      const top = html.scrollTop || body.scrollTop;
      const scrollHeight = Math.max(
        body.scrollHeight,
        body.offsetHeight,
        html.clientHeight,
        html.scrollHeight,
        html.offsetHeight
      );
      const clientHeight = html.clientHeight;
      const maxScroll = Math.max(1, scrollHeight - clientHeight);
      return { top, maxScroll };
    } catch {
      return { top: 0, maxScroll: 1 };
    }
  }

  function syncEditorToIframe() {
    const view = editorViewRef.value;
    const iframe = previewIframeRef.value;
    if (!view || !iframe) return;

    const { top, maxScroll } = getEditorScrollInfo(view);
    const pct = top / maxScroll;

    if (Math.abs(pct - iframeScrollPct.value) < 0.02) return;

    isSyncing.value = true;
    editorScrollPct.value = pct;

    try {
      const doc = iframe.contentDocument || iframe.contentWindow?.document;
      if (doc) {
        const html = doc.documentElement;
        const body = doc.body;
        const scrollHeight = Math.max(
          body.scrollHeight,
          body.offsetHeight,
          html.clientHeight,
          html.scrollHeight,
          html.offsetHeight
        );
        const clientHeight = html.clientHeight;
        const maxIframeScroll = Math.max(1, scrollHeight - clientHeight);
        const targetTop = pct * maxIframeScroll;
        html.scrollTop = targetTop;
        body.scrollTop = targetTop;
        iframeScrollPct.value = targetTop / maxIframeScroll;
      }
    } catch {
      // Cross-origin or iframe not ready
    }

    setTimeout(() => {
      isSyncing.value = false;
    }, debounceMs);
  }

  function syncIframeToEditor() {
    const view = editorViewRef.value;
    const iframe = previewIframeRef.value;
    if (!view || !iframe) return;

    const { top, maxScroll } = getIframeScrollInfo(iframe);
    const pct = top / maxScroll;

    if (Math.abs(pct - editorScrollPct.value) < 0.02) return;

    isSyncing.value = true;
    iframeScrollPct.value = pct;

    const scrollDOM = view.scrollDOM;
    const editorMaxScroll = Math.max(1, scrollDOM.scrollHeight - scrollDOM.clientHeight);
    const targetTop = pct * editorMaxScroll;
    scrollDOM.scrollTop = targetTop;
    editorScrollPct.value = targetTop / editorMaxScroll;

    setTimeout(() => {
      isSyncing.value = false;
    }, debounceMs);
  }

  function attachListeners() {
    detachListeners();

    const view = editorViewRef.value;
    const iframe = previewIframeRef.value;
    if (!view || !iframe) return;

    const scrollDOM = view.scrollDOM;
    const onEditorScroll = () => {
      if (isSyncing.value) return;
      syncEditorToIframe();
    };
    scrollDOM.addEventListener("scroll", onEditorScroll);
    editorListener = () => scrollDOM.removeEventListener("scroll", onEditorScroll);

    // Use a MutationObserver to detect when iframe content is loaded
    // and attach scroll listener to the iframe's document
    const checkIframe = () => {
      try {
        const doc = iframe.contentDocument || iframe.contentWindow?.document;
        if (doc) {
          const onIframeScroll = () => {
            if (isSyncing.value) return;
            syncIframeToEditor();
          };
          doc.addEventListener("scroll", onIframeScroll, true);
          iframeListener = () => doc.removeEventListener("scroll", onIframeScroll, true);
          return true;
        }
      } catch {
        // iframe not ready yet
      }
      return false;
    };

    // Try immediately and also after load
    if (!checkIframe()) {
      iframe.addEventListener("load", () => {
        checkIframe();
      });
    }
  }

  function detachListeners() {
    if (editorListener) {
      editorListener();
      editorListener = null;
    }
    if (iframeListener) {
      iframeListener();
      iframeListener = null;
    }
  }

  // Watch for editor and iframe refs becoming available
  watch(
    [editorViewRef, previewIframeRef],
    ([newEditor, newIframe]) => {
      if (newEditor && newIframe) {
        // Give DOM time to settle
        setTimeout(attachListeners, 300);
      }
    },
    { immediate: false }
  );

  onBeforeUnmount(() => {
    detachListeners();
  });
}
