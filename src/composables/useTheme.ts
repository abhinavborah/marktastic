import { ref, watch, onMounted } from "vue";
import type { ThemeMode } from "../types";

const STORAGE_KEY = "marktastic-theme";

const theme = ref<ThemeMode>("system");
const isDark = ref(false);

function detectSystemDark(): boolean {
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function applyTheme(mode: ThemeMode) {
  const html = document.documentElement;
  let dark = false;

  if (mode === "system") {
    dark = detectSystemDark();
  } else {
    dark = mode === "dark";
  }

  isDark.value = dark;

  if (dark) {
    html.classList.add("dark");
  } else {
    html.classList.remove("dark");
  }
}

export function useTheme() {
  onMounted(() => {
    const stored = localStorage.getItem(STORAGE_KEY) as ThemeMode | null;
    if (stored && ["light", "dark", "system"].includes(stored)) {
      theme.value = stored;
    }
    applyTheme(theme.value);

    // Watch for system preference changes when in system mode
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", () => {
      if (theme.value === "system") {
        applyTheme("system");
      }
    });
  });

  watch(theme, (newMode) => {
    localStorage.setItem(STORAGE_KEY, newMode);
    applyTheme(newMode);
  });

  function setTheme(mode: ThemeMode) {
    theme.value = mode;
  }

  function cycleTheme() {
    const modes: ThemeMode[] = ["light", "dark", "system"];
    const currentIndex = modes.indexOf(theme.value);
    const nextIndex = (currentIndex + 1) % modes.length;
    setTheme(modes[nextIndex]);
  }

  return {
    theme,
    isDark,
    setTheme,
    cycleTheme,
  };
}
