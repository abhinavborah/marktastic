import { ref, watch, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { useToast } from "./useToast";

const pdfUrl = ref<string | null>(null);
const pdfBytes = ref<Uint8Array | null>(null);
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
  toastApi?: ReturnType<typeof useToast>
) {
  pdfLoading.value = true;
  lastError.value = null;

  if (toastApi) {
    toastApi.info("Compiling PDF...", 2000);
  }

  try {
    const bytes = await invoke<number[]>("convert_md_to_pdf", {
      markdown,
      templateName,
    });

    const uint8Array = new Uint8Array(bytes);
    pdfBytes.value = uint8Array;
    const blob = new Blob([uint8Array], { type: "application/pdf" });
    revokeCurrentUrl();

    currentBlobUrl = URL.createObjectURL(blob);
    pdfUrl.value = currentBlobUrl;

    if (toastApi) {
      toastApi.success("PDF ready", 2000);
    }
  } catch (err) {
    lastError.value = String(err);
    console.error("PDF generation failed:", err);
    if (toastApi) {
      toastApi.error(`PDF compilation failed: ${err}`, 5000);
    }
  } finally {
    pdfLoading.value = false;
  }
}

export function usePdf(
  markdownRef: Ref<string>,
  templateRef: Ref<string>,
  toastApi?: ReturnType<typeof useToast>,
  debounceMs = 500
) {
  watch(
    [markdownRef, templateRef],
    () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }

      // Skip compilation if markdown is empty
      if (!markdownRef.value || markdownRef.value.trim().length === 0) {
        pdfLoading.value = false;
        lastError.value = null;
        pdfUrl.value = null;
        pdfBytes.value = null;
        revokeCurrentUrl();
        return;
      }

      debounceTimer = setTimeout(() => {
        generatePdf(markdownRef.value, templateRef.value, toastApi);
      }, debounceMs);
    },
    { immediate: true }
  );

  return {
    pdfUrl,
    pdfBytes,
    pdfLoading,
    lastError,
    revokeCurrentUrl,
  };
}
