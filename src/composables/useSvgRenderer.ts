import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pages = ref<string[]>([]);
const totalPages = ref(0);
const rendering = ref(false);
const renderError = ref<string | null>(null);
const isRecompiling = ref(false);

export function useSvgRenderer(
  editorContentRef: Ref<string>,
  selectedTemplateRef: Ref<string>
) {
  async function render() {
    if (!editorContentRef.value.trim()) {
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
        markdown: editorContentRef.value,
        templateName: selectedTemplateRef.value,
      });
      pages.value = result;
      totalPages.value = result.length;
    } catch (err: any) {
      renderError.value = String(err);
      console.error("SVG rendering failed:", err);
    } finally {
      rendering.value = false;
      isRecompiling.value = false;
    }
  }

  // Debounced watch on editor content + template
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
