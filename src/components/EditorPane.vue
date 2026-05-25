<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, shallowRef } from "vue";
import { EditorView, keymap, scrollPastEnd } from "@codemirror/view";
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
  wordWrap?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "editorReady", view: EditorView): void;
}>();

const editorContainer = ref<HTMLDivElement | null>(null);
const view = shallowRef<EditorView | null>(null);
const themeCompartment = new Compartment();
const wrapCompartment = new Compartment();

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
          minHeight: "100%",
        },
        ".cm-line": {
          padding: "0 16px",
        },
        ".cm-gutters": {
          borderRight: "1px solid hsl(var(--border) / 1)",
        },
        ".cm-lineNumbers .cm-gutterElement": {
          color: "hsl(var(--muted-foreground) / 1)",
          padding: "0 8px",
          minWidth: "2.5em",
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
    scrollPastEnd(),
    EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        emit("update:modelValue", update.state.doc.toString());
      }
    }),
    themeCompartment.of(getThemeExtensions(isDark)),
    wrapCompartment.of(props.wordWrap ? EditorView.lineWrapping : []),
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
    emit("editorReady", view.value);
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

// Toggle word wrap when prop changes
watch(
  () => props.wordWrap,
  (newVal) => {
    if (!view.value) return;
    view.value.dispatch({
      effects: wrapCompartment.reconfigure(newVal ? EditorView.lineWrapping : []),
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
