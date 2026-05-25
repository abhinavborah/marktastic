<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, shallowRef } from "vue";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState, Compartment } from "@codemirror/state";
import { markdown } from "@codemirror/lang-markdown";
import { oneDark } from "@codemirror/theme-one-dark";
import { defaultKeymap } from "@codemirror/commands";
import { lineNumbers } from "@codemirror/view";
import { syntaxHighlighting, defaultHighlightStyle } from "@codemirror/language";
import type { ThemeMode } from "../types";

const props = defineProps<{
  modelValue: string;
  theme: ThemeMode;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const editorContainer = ref<HTMLDivElement | null>(null);
const view = shallowRef<EditorView | null>(null);
const themeCompartment = new Compartment();

function getThemeExtensions(isDark: boolean) {
  const extensions: any[] = [
    EditorView.theme(
      {
        "&": {
          fontSize: "14px",
          fontFamily:
            'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
        },
        ".cm-content": {
          padding: "12px 16px",
          minHeight: "100%",
        },
        ".cm-gutters": {
          padding: "12px 0",
          borderRight: "1px solid var(--border)",
          backgroundColor: "transparent",
        },
        ".cm-lineNumbers .cm-gutterElement": {
          color: "hsl(var(--muted-foreground))",
          padding: "0 8px",
        },
      },
      { dark: isDark }
    ),
  ];

  if (isDark) {
    extensions.push(oneDark);
  }

  return extensions;
}

function getExtensions(theme: ThemeMode) {
  const isDark =
    theme === "dark" ||
    (theme === "system" &&
      window.matchMedia("(prefers-color-scheme: dark)").matches);

  return [
    keymap.of(defaultKeymap),
    lineNumbers(),
    markdown(),
    syntaxHighlighting(defaultHighlightStyle),
    EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        emit("update:modelValue", update.state.doc.toString());
      }
    }),
    themeCompartment.of(getThemeExtensions(isDark)),
  ];
}

function createView(content: string, theme: ThemeMode) {
  view.value?.destroy();

  const state = EditorState.create({
    doc: content,
    extensions: getExtensions(theme),
  });

  if (editorContainer.value) {
    view.value = new EditorView({
      state,
      parent: editorContainer.value,
    });
  }
}

onMounted(() => {
  createView(props.modelValue, props.theme);
});

onBeforeUnmount(() => {
  view.value?.destroy();
});

// Update theme when it changes
watch(
  () => props.theme,
  (newTheme) => {
    if (!view.value) return;
    const isDark =
      newTheme === "dark" ||
      (newTheme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);
    view.value.dispatch({
      effects: themeCompartment.reconfigure(getThemeExtensions(isDark)),
    });
  }
);

// Sync external modelValue changes into editor
watch(
  () => props.modelValue,
  (newValue) => {
    if (!view.value) return;
    const current = view.value.state.doc.toString();
    if (newValue !== current) {
      view.value.dispatch({
        changes: { from: 0, to: current.length, insert: newValue },
      });
    }
  }
);
</script>

<template>
  <div ref="editorContainer" class="h-full overflow-hidden" />
</template>
