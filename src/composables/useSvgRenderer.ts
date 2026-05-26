import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pages = ref<string[]>([]);
const totalPages = ref(0);
const rendering = ref(false);
const renderError = ref<string | null>(null);
const isRecompiling = ref(false);

// Track last rendered content to skip recompile of identical documents
let lastRenderedContent = "";
let lastRenderedTemplate = "";

export function useSvgRenderer(
  editorContentRef: Ref<string>,
  selectedTemplateRef: Ref<string>
) {
  async function render() {
    const content = editorContentRef.value;
    const template = selectedTemplateRef.value;

    // Dedupe: skip if content and template unchanged since last render
    if (content === lastRenderedContent && template === lastRenderedTemplate) {
      return;
    }

    if (!content.trim()) {
      pages.value = [];
      totalPages.value = 0;
      isRecompiling.value = false;
      rendering.value = false;
      return;
    }

    isRecompiling.value = true;
    rendering.value = true;
    renderError.value = null;

    try {
      const result = await invoke<string[]>("convert_md_to_svg", {
        markdown: content,
        templateName: template,
      });

      // Yield to browser so CodeMirror can process pending keystrokes
      // before we block the main thread with DOM updates
      await new Promise<void>((resolve) => {
        if (typeof (window as any).requestIdleCallback === "function") {
          (window as any).requestIdleCallback(() => resolve(), { timeout: 50 });
        } else {
          setTimeout(resolve, 0);
        }
      });

      pages.value = result;
      totalPages.value = result.length;
      lastRenderedContent = content;
      lastRenderedTemplate = template;
    } catch (err: any) {
      renderError.value = String(err);
      console.error("SVG rendering failed:", err);
    } finally {
      rendering.value = false;
      isRecompiling.value = false;
    }
  }

  // Debounced watch on editor content + template (manual debounce, not Vue API)
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  watch(
    [editorContentRef, selectedTemplateRef],
    () => {
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        render();
      }, 500);
    },
    { immediate: false }
  );

  return {
    pages,
    totalPages,
    rendering,
    renderError,
    isRecompiling,
  };
}
