import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pdfUrl = ref<string | null>(null);
const pdfLoading = ref(false);
const lastError = ref<string | null>(null);

let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let currentBlobUrl: string | null = null;

function revokeCurrentUrl() {
  if (currentBlobUrl) {
    URL.revokeObjectURL(currentBlobUrl);
    currentBlobUrl = null;
  }
}

async function generatePdf(
  markdown: string,
  templateName: string,
  command: string,
  folderPath: string | null
) {
  pdfLoading.value = true;
  lastError.value = null;

  try {
    let bytes: number[];

    if (command === "compile_folder_to_pdf" && folderPath) {
      bytes = await invoke<number[]>("compile_folder_to_pdf", {
        folderPath,
        templateName,
      });
    } else {
      bytes = await invoke<number[]>("convert_md_to_pdf", {
        markdown,
        templateName,
      });
    }

    const uint8Array = new Uint8Array(bytes);
    const blob = new Blob([uint8Array], { type: "application/pdf" });
    revokeCurrentUrl();

    currentBlobUrl = URL.createObjectURL(blob);
    pdfUrl.value = currentBlobUrl;
  } catch (err) {
    lastError.value = String(err);
    console.error("PDF generation failed:", err);
  } finally {
    pdfLoading.value = false;
  }
}

export function usePdf(
  markdownRef: Ref<string>,
  templateRef: Ref<string>,
  commandRef: Ref<string>,
  folderPathRef: Ref<string | null>,
  debounceMs = 500
) {
  watch(
    [markdownRef, templateRef, commandRef, folderPathRef],
    () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      debounceTimer = setTimeout(() => {
        generatePdf(
          markdownRef.value,
          templateRef.value,
          commandRef.value,
          folderPathRef.value
        );
      }, debounceMs);
    },
    { immediate: true }
  );

  return {
    pdfUrl,
    pdfLoading,
    lastError,
    revokeCurrentUrl,
  };
}
