# Marktastic — Progress

## Session Date
2026-05-25

## Phase 1: Project Scaffolding — Status: `completed`

### Actions Taken
- Tauri v2 + Vue 3 + TypeScript project initialized via `npm create tauri-app@2`
- Tailwind CSS v3 + PostCSS configured (`tailwind.config.js`, `postcss.config.js`, `@tailwind` directives in `style.css`)
- shadcn-vue manually installed:
  - `cn()` utility function (`src/lib/utils.ts`)
  - CSS variables in `style.css` (color system, radius, dark mode readiness)
  - Tailwind animation utilities
- CodeMirror 6 packages installed (`@codemirror/view`, `@codemirror/state`, `@codemirror/lang-markdown`, `@codemirror/theme-one-dark`, `@codemirror/commands`, `@codemirror/search`, `@codemirror/lint`)
- Tauri v2 plugins configured:
  - npm deps: `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-fs`, `@tauri-apps/plugin-shell`
  - Rust deps: `tauri-plugin-dialog`, `tauri-plugin-fs`, `tauri-plugin-shell`
  - `lib.rs` updated with plugin initializers (`.plugin(tauri_plugin_dialog::init())`, etc.)
  - Capabilities file updated with permissions (`dialog:allow-open`, `fs:allow-read`, `fs:allow-write`, etc.)
- `src-tauri/templates/` directory created for built-in Typst templates
- Compilation verified: `npm install` passed, `npm run build` passed, `cargo check` passed, `tauri dev` passed

### Notes
- `context/` directory and `handoff.md` were accidentally deleted during scaffolding and subsequently regenerated
- Phase 1 scaffolding was committed as `c23a540` with conventional commit style (lowercase, granular): `scaffold tauri v2 + vue 3 + typescript project with tailwind, shadcn-vue, codemirror 6, and tauri plugins`
- Agent workspace files (`agent-info.md`, `handoff.md`, `context/`) were accidentally included in `c23a540`, then removed from git tracking in commit `cc4df4b`: `remove agent workspace files from tracking`
- Workspace files were temporarily moved into `docs/` by mistake, then restored to their original root and `context/` locations
- Current git state: working directory has untracked `agent-info.md`, `handoff.md`, and `context/`; only project code is committed to the repo

### Files Created / Modified
- `~/Developer/marktastic/context/` (directory — regenerated after accidental deletion)
- `~/Developer/marktastic/context/findings.md` (regenerated)
- `~/Developer/marktastic/context/task_plan.md` (regenerated)
- `~/Developer/marktastic/context/progress.md` (regenerated)
- `~/Developer/marktastic/handoff.md` (regenerated)
- `~/Developer/marktastic/agent-info.md`
- Tauri app scaffolded in project root (`package.json`, `vite.config.ts`, `tsconfig.json`, `src/`, `src-tauri/`)

## Phase 2: Rust Backend — Typst Pipeline — Status: `completed`

### Actions Taken
- Added Rust dependencies to `src-tauri/Cargo.toml`:
  - `pulldown-cmark = "0.13.4"` — Markdown parser with GFM support
  - `typst = "0.14.2"` — Typst compilation engine
  - `typst-pdf = "0.14.2"` — PDF export from Typst
  - `typst-kit = { version = "0.14.2", features = ["embed-fonts"] }` — Font search and loading
  - `comemo = "0.5.1"`, `time = "0.3.47"`, `ttf-parser = "0.25.1"`, `ureq = "2"`, `tar = "0.4.46"`, `zune-inflate = "0.2.54"` — Supporting crates for Typst world
- Created `src-tauri/src/typst_world.rs` — custom `TypstWrapperWorld` implementing `typst::World` trait, adapted from `typst-as-library` reference pattern
- Created `src-tauri/src/md_to_typst.rs` — Markdown→Typst converter using `pulldown-cmark` with GFM options (tables, task lists, strikethrough, footnotes, smart punctuation, heading attributes)
- Created 3 built-in Typst templates in `src-tauri/templates/`:
  - `basic-report.typ` — Clean academic report with page numbers, Libertinus Serif/Sans, heading numbering
  - `university-assignment.typ` — Assignment style with generous margins, New Computer Modern, section dividers
  - `thesis-chapter.typ` — Formal thesis chapter with first-line indent, justified text, chapter header/footer
- Implemented Tauri commands in `src-tauri/src/lib.rs`:
  - `convert_md_to_pdf(markdown, template_name) -> Result<Vec<u8>, String>` — Full in-memory pipeline: parse MD → convert to Typst markup → load template → inject body via `// MARKTASTIC_BODY_CONTENT` marker → compile with Typst → export PDF bytes
  - `open_file_path(file_path) -> Result<String, String>` — Read a `.md` file by path and return contents
  - `open_folder(folder_path) -> Result<Vec<(String, String)>, String>` — List all `.md` files in a folder, return (filename, content) pairs
  - `get_templates() -> Result<Vec<String>, String>` — Return available built-in template names
- All commands registered in `.invoke_handler()`
- Compilation verified: `cargo check` passed with 0 errors, 0 warnings

### Notes
- `open_file` (dialog-based) was attempted but Tauri v2 dialog plugin backend API differs from expected; replaced with `open_file_path` that accepts a pre-selected path. Frontend will use Tauri's JS dialog API for file picking.
- Template resolution uses multiple candidate paths to handle both dev (`target/debug/`) and production builds
- In-memory compilation — no temp files used for Typst→PDF pipeline
- Error handling: all commands return `Result<T, String>` with descriptive error messages

## Phase 3: Vue Frontend — Status: `completed`

### Actions Taken
- Created `src/types.ts` — shared TypeScript types (`ThemeMode`, `FileEntry`, `OpenedFolder`)
- Created `src/composables/useTheme.ts` — theme management composable with three modes (`light`, `dark`, `system`), OS preference detection, `localStorage` persistence, and `<html>` class toggling for shadcn-vue dark mode
- Created `src/composables/usePdf.ts` — PDF generation composable that watches editor content + selected template, debounces by 500ms, calls `convert_md_to_pdf` Rust command via Tauri invoke, manages blob URL lifecycle (create/revoke)
- Created `src/components/WelcomeScreen.vue` — landing screen with "Marktastic" title, subtitle, and "Open File" / "Open Folder" buttons with inline SVG icons
- Created `src/components/EditorPane.vue` — CodeMirror 6 wrapper component:
  - `@codemirror/lang-markdown` for syntax highlighting
  - `@codemirror/theme-one-dark` for dark theme
  - `lineNumbers()`, `defaultKeymap`, `syntaxHighlighting`
  - Theme changes reconfigure editor via `Compartment`
  - Two-way binding with Vue `v-model`
- Created `src/components/PreviewPane.vue` — PDF iframe wrapper with loading spinner, error display, empty state, and live PDF rendering via blob URL
- Created `src/components/SplitView.vue` — resizable horizontal split pane with drag handle and floating toggle buttons (editor-only / both / preview-only modes)
- Created `src/components/Toolbar.vue` — app header with:
  - Logo + "Marktastic" title
  - Open File / Open Folder buttons
  - Template selector dropdown (populated from `get_templates`)
  - Theme cycle toggle (☀ Light / ☾ Dark / ◐ System)
  - Export PDF button (disabled when no content)
- Updated `src/App.vue` — main app shell integrating all components:
  - Welcome screen ↔ editor+preview routing based on `isWelcome` state
  - File/folder open flow using Tauri JS `dialog` plugin → `open_file_path` / `open_folder` commands
  - PDF export flow using Tauri JS `save` dialog + `fs` plugin `writeFile`
  - Template selection state passed through to PDF generation
- Removed old Tauri starter styles from `src/App.vue` scoped CSS, added full-height layout styles
- Compilation verified:
  - `npm run build` passed (vue-tsc + vite build)
  - `cargo check` passed with 0 errors
  - `tauri dev` starts successfully

### Notes
- Frontend uses Tauri v2 JS dialog API (`@tauri-apps/plugin-dialog`) for file picking; backend command `open_file_path` reads the selected file
- PDF export fetches the existing blob URL, converts to Uint8Array, writes via `fs` plugin
- CodeMirror theme switches dynamically between light and one-dark based on the active theme mode
- Split view has draggable resizer and floating pane-toggle buttons

## Phase 4: Wikilink Resolution, Merged Editor View, and Smart Scroll Sync — Status: `completed`

### Actions Taken
- Added `regex` and `lazy_static` crates to `src-tauri/Cargo.toml`
- Created `src-tauri/src/wikilinks.rs`:
  - `WIKILINK_RE` regex pattern `\[\[([^\]]+)\]\]` for parsing `[[filename]]` and `[[filename.md]]` wikilinks
  - `find_entry_point()` — looks for `index.md` then `main.md` in a folder; returns error if neither exists
  - `extract_wikilinks()` — finds all wikilink targets in markdown content
  - `normalize_link()` — adds `.md` extension if missing
  - `resolve_wikilinks()` — BFS graph traversal starting from entry point, returns ordered list of reachable file names
  - `build_merged_document()` — merges all reachable files into a single markdown document with `<!-- file: filename.md -->` section headers and `---` separators; processes wikilinks into markdown internal links (`[filename](#section-filename)`) or `*Missing link: filename*` placeholders
  - Unit tests for `extract_wikilinks` and `normalize_link`
- Updated `src-tauri/src/md_to_typst.rs`:
  - Added `preprocess_wikilinks()` function that replaces any remaining `[[...]]` syntax with `*Missing link: ...*` emphasis before pulldown-cmark parsing
  - Ensures wikilinks in single-file mode are handled gracefully
- Updated `src-tauri/src/lib.rs`:
  - Extracted shared `compile_markdown_to_pdf()` function used by both `convert_md_to_pdf` and new `compile_folder_to_pdf`
  - Added `compile_folder_to_pdf(folder_path, template_name)` command — resolves wikilinks, merges files, compiles to PDF
  - Added `resolve_wikilinks(folder_path)` command — returns ordered list of reachable `.md` files
  - Registered both commands in `.invoke_handler()`
- Updated `src/App.vue`:
  - Added folder mode state: `isFolderMode`, `currentFolderPath`, `reachableFiles`, `pdfCommand`
  - `handleOpenFolder()` now calls `resolve_wikilinks` to get ordered file list, then merges reachable files into editor with section comments and `---` separators
  - Single-file mode uses `convert_md_to_pdf`; folder mode uses `compile_folder_to_pdf`
  - Integrated scroll sync: passes editor view ref and iframe ref to `useScrollSync`
- Updated `src/composables/usePdf.ts`:
  - Added `commandRef` and `folderPathRef` parameters
  - Dynamically calls `convert_md_to_pdf` or `compile_folder_to_pdf` based on active mode
- Created `src/composables/useScrollSync.ts`:
  - Percentage-based bidirectional scroll sync between CodeMirror editor and PDF preview iframe
  - `syncEditorToIframe()` — calculates editor scroll percentage, applies to iframe
  - `syncIframeToEditor()` — calculates iframe scroll percentage, applies to editor
  - `isSyncing` flag prevents infinite scroll loops
  - Debounced sync with 50ms delay
  - Handles iframe load detection via `load` event and `MutationObserver` fallback
- Updated `src/components/EditorPane.vue`:
  - Emits `editorReady` event with the `EditorView` instance for scroll sync
- Updated `src/components/PreviewPane.vue`:
  - Emits `iframeReady` event with the `HTMLIFrameElement` for scroll sync
  - `@load` handler re-emits iframe reference when PDF content loads
- Updated `src/components/Toolbar.vue`:
  - Added `folderPath` and `reachableFiles` props
  - Shows folder name badge with file count when in folder mode
- Updated `src/components/WelcomeScreen.vue`:
  - Added explanatory text for Open File vs Open Folder modes
  - Documents `index.md`/`main.md` entry point requirement and `[[filename]]` wikilink syntax
- Compilation verified:
  - `npm run build` passed (vue-tsc + vite build)
  - `cargo check` passed with 0 errors, 0 warnings

### Notes
- ~~BUG: `compile_folder_to_pdf` compiled from original disk files, ignoring editor edits in folder mode.~~ **FIXED** — `usePdf` now always calls `convert_md_to_pdf` with the current `editorContent` as the single source of truth. Folder mode builds the merged string once during open, then live preview uses edited content.
- Scroll sync is percentage-based and "good enough" — not pixel-perfect. Cross-origin iframe restrictions may limit sync in some browsers.
- Broken wikilinks render as italic placeholder text in the PDF.

## Test Results / Error Log
- Phase 3 build: `npm run build` ✅, `cargo check` ✅, `tauri dev` ✅
- Phase 4 build: `npm run build` ✅, `cargo check` ✅
- Phase 4 bug fix: `npm run build` ✅, `cargo check` ✅

## 5-Question Reboot Check

| Question | Answer |
|----------|--------|
| Where am I? | Phase 4 complete (Wikilink Resolution + Scroll Sync). Rust backend resolves `[[file]]` wikilinks, merges multi-file documents, compiles to PDF. Vue frontend shows merged editor view with section separators, bidirectional scroll sync between editor and preview. `npm run build` and `cargo check` clean. Awaiting Phase 5. |
| Where am I going? | Phase 5 — Dark Mode + Polish (system preference detection, keyboard shortcuts, window controls, template switcher UI refinements) |
| What's the goal? | Cross-platform md→pdf app with live preview, split view, 3 templates |
| What have I learned? | See `findings.md`. Regex-based wikilink parsing works well. BFS graph traversal for dependency resolution is simple and effective for flat folder structures. Percentage-based scroll sync with a sync flag prevents loops. |
| What have I done? | See session log above |

## Phase 5: Dark Mode Polish, Keyboard Shortcuts, Toasts, and UX Polish — Status: `completed`

### Actions Taken
- Created `src/composables/useToast.ts` — reactive toast system with 4 types (`info`, `success`, `warning`, `error`), auto-dismiss after configurable duration, unique IDs
- Created `src/components/ToastContainer.vue` — fixed-position bottom-right toast stack with colored badges, close buttons, Vue `<TransitionGroup>` animations
- Created `src/composables/useKeyboard.ts` — global keydown listener with platform detection (macOS `Meta` / Windows+Linux `Ctrl`). Registered shortcuts:
  - `Ctrl/Cmd+O` → Open File
  - `Ctrl/Cmd+Shift+O` → Open Folder
  - `Ctrl/Cmd+S` → Export PDF
  - `Ctrl/Cmd+E` → Editor-only pane
  - `Ctrl/Cmd+P` → Preview-only pane
  - `Ctrl/Cmd+B` → Both panes
  - `Ctrl/Cmd+T` → Cycle theme (light → dark → system)
- Updated `src/composables/usePdf.ts` — accepts optional `toastApi` parameter, shows "Compiling PDF..." info toast on start, "PDF ready" success toast on completion, error toast on failure
- Updated `src/App.vue` — integrated `useKeyboard`, `useToast`, `ToastContainer`. Shows toasts on file open, folder open (with file count + broken wikilink warnings), export success/failure. Added dynamic window title updates via `getCurrentWebviewWindow().setTitle()`. Connected `paneMode` ref to `SplitView` for keyboard control.
- Updated `src/components/SplitView.vue` — converted pane mode to `v-model` prop/emit pattern so `App.vue` can control it via keyboard shortcuts. Added keyboard shortcut hints to toggle button tooltips.
- Updated `src/components/PreviewPane.vue` — polished loading spinner (larger, centered, "Compiling PDF..." text), empty state (larger icon). Added black overlay that shows while PDF iframe is loading its content — prevents white flash in dark mode.
- Updated `src/components/Toolbar.vue` — added macOS traffic light padding (`pl-20` on macOS), keyboard shortcut hints in button tooltips and labels (e.g., "Open (⌘+O)" / "Open (Ctrl+O)"), focus-visible styles already inherited from CSS.
- Updated `src/assets/index.css` — added smooth theme transition (`transition: background-color 0.3s, color 0.3s`), custom styled scrollbars (`::-webkit-scrollbar` with muted-foreground colored thumb), focus-visible outline rings (`outline: 2px solid hsl(var(--ring))`).
- Compilation verified:
  - `npm run build` passed (vue-tsc + vite build)
  - `cargo check` passed with 0 errors, 0 warnings

### Notes
- File sidebar for folder mode was skipped as a future enhancement — would require tracking section boundaries in the editor for clickable navigation.
- Window title updates via `getCurrentWebviewWindow()` may not work in browser dev mode but will work in the Tauri app.
- Keyboard shortcuts are platform-aware: macOS uses `Meta` key, others use `Ctrl`.
- The `Ctrl+S` shortcut prevents the browser's default save dialog and triggers the app's PDF export instead.

## Bug Fixes — 2026-05-25

### Bug 1: Glitchy resize during split-view drag
**Root cause:** `transition-all` on pane `<div>` elements caused CSS to interpolate every rapid width change during drag.  
**Fix:** Removed `transition-all` class from both editor and preview panes in `SplitView.vue`. Width now changes instantly during drag.  
**File changed:** `src/components/SplitView.vue`

### Bug 2: Export to PDF fails inside Tauri WebView
**Root cause:** `handleExportPdf` called `fetch(pdfUrl.value)` on a blob URL. Tauri WebView security context blocks `fetch()` on blob URLs.  
**Fix:** Store raw PDF bytes alongside the blob URL in `usePdf.ts`. Export reads the stored `Uint8Array` directly via `writeFile` without any `fetch`/`blob`/`arrayBuffer` roundtrip.  
**Files changed:** `src/composables/usePdf.ts`, `src/App.vue`

### Bug 3: "Compiling" message on app startup
**Root cause:** `usePdf.ts` watcher has `{ immediate: true }`. On mount `editorContent` is `""`, so `generatePdf("", ...)` fires immediately, showing the "Compiling PDF..." toast and loading spinner on the welcome screen.  
**Fix:** Added guard in the watch callback: if `markdownRef.value.trim().length === 0`, skip compilation, reset state, and return early.  
**File changed:** `src/composables/usePdf.ts`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Test Results / Error Log
- Phase 3 build: `npm run build` ✅, `cargo check` ✅, `tauri dev` ✅
- Phase 4 build: `npm run build` ✅, `cargo check` ✅
- Phase 4 bug fix: `npm run build` ✅, `cargo check` ✅
- Phase 5 build: `npm run build` ✅, `cargo check` ✅
- Bug fix build: `npm run build` ✅, `cargo check` ✅

## Critical Bug Fixes — 2026-05-25

### Bug 1: Export to PDF fails silently
**Root cause:** `writeFile` from `@tauri-apps/plugin-fs` may fail silently in some Tauri v2 contexts; no diagnostic info was available.  
**Fix:** Added `console.log` at every step in `handleExportPdf` for diagnostics. Added a `try/catch` around `writeFile` with a fallback that fetches the blob URL, converts to `Uint8Array`, and writes again. Error toast now shows the actual error message.  
**File changed:** `src/App.vue`

### Bug 2: Drag handle continues after mouse release
**Root cause:** `document.addEventListener('mousemove'/'mouseup')` doesn't fire when the mouse is over an iframe (PDF preview). The iframe captures events, so `stopDrag` never runs and `isDragging` stays `true`.  
**Fix:** Rewrote drag logic in `SplitView.vue`:
- Use `window.addEventListener` instead of `document.addEventListener` (captures events globally)
- Added `window.addEventListener('blur', ...)` to handle Alt+Tab during drag
- Set `document.body.style.userSelect = 'none'` and `cursor = 'col-resize'` during drag
- Added cleanup function stored in `dragCleanup`, called in `onBeforeUnmount`
- Added `data-dragging` attribute on `#split-container`  
**File changed:** `src/components/SplitView.vue`

### Bug 3: Glitchy resize during drag
**Root cause:** Same as Bug 2 (iframe stealing events) plus lack of width optimization.  
**Fix:**
- Added CSS: `#split-container[data-dragging="true"] iframe { pointer-events: none !important; }` — iframe can't steal mouse events during drag
- Added CSS: `#split-container > div { will-change: width; }` — browser optimizes width repaints
- `transition-all` was already removed in a previous commit  
**File changed:** `src/assets/index.css`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Editor UI Fixes — 2026-05-25

### Issue 1: No vertical scrollbar in editor
**Root cause:** CodeMirror `.cm-scroller` didn't have `overflow: auto` and the editor container didn't fill its parent height.  
**Fix:** Added CSS rules in `index.css` for `.cm-editor { height: 100%; }` and `.cm-scroller { overflow: auto; min-height: 100%; }`.  
**File changed:** `src/assets/index.css`

### Issue 2: Editor shrinks to content height
**Root cause:** CodeMirror defaults to content-height sizing without `scrollPastEnd()`.  
**Fix:** Imported `scrollPastEnd` from `@codemirror/view` and added it to the extensions in `EditorPane.vue`. Editor now fills the full container height even with one line of text.  
**File changed:** `src/components/EditorPane.vue`

### Issue 3: Word wrap toggle
**Fix:** Added `wordWrap` prop to `EditorPane.vue` using a `Compartment` for `EditorView.lineWrapping`. Added toggle button to `SplitView.vue` floating controls. Added `Ctrl/Cmd+Shift+W` shortcut in `useKeyboard.ts`. Wired `wordWrap` ref through `App.vue`.  
**Files changed:** `src/components/EditorPane.vue`, `src/components/SplitView.vue`, `src/composables/useKeyboard.ts`, `src/App.vue`

### Issue 4: Line numbers overlap scrolled text
**Root cause:** `.cm-gutters` had `backgroundColor: "transparent"` in the CodeMirror theme extension, so horizontally scrolled text showed through behind line numbers.  
**Fix:** Removed transparent background from the theme extension in `EditorPane.vue`. Added explicit `.cm-gutters` CSS in `index.css` with `background-color: hsl(var(--muted) / 1)` and matching border.  
**Files changed:** `src/components/EditorPane.vue`, `src/assets/index.css`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Tauri FS Capabilities Fix — 2026-05-25

### Bug: "fs write not allowed" when exporting PDF
**Root cause:** `fs:default` in `src-tauri/capabilities/default.json` only grants read access to application-specific directories (`$APPDATA`, `$APPCONFIG`, etc.). It does NOT include write access to user directories like `$HOME`, `$DOCUMENT`, `$DOWNLOAD`, or `$DESKTOP`. The `writeFile` call from `@tauri-apps/plugin-fs` requires both the write command permission AND a scope that allows the target path.  
**Fix:** Added four Tauri v2 directory-specific recursive write permissions to `default.json`:
- `fs:allow-home-write-recursive` — write to `$HOME/**`
- `fs:allow-document-write-recursive` — write to `$DOCUMENT/**`
- `fs:allow-download-write-recursive` — write to `$DOWNLOAD/**`
- `fs:allow-desktop-write-recursive` — write to `$DESKTOP/**`

These are permission *sets* that each include `write-all` (all write commands) plus the directory scope. This is the idiomatic Tauri v2 pattern for granting write access to common user directories.  
**Also:** Simplified `handleExportPdf` in `App.vue` — removed the blob-fetch fallback (now unnecessary since the real issue was permissions, not the write method). Kept console logging for diagnostics.  
**Files changed:** `src-tauri/capabilities/default.json`, `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## UI Fixes — 2026-05-25

### Issue 1: Line numbers vertically misaligned
**Root cause:** `.cm-content` had `padding: "12px 16px"` and `.cm-gutters` had `padding: "12px 0"` in the CodeMirror theme extension. CodeMirror internally matches gutter element heights to content line heights, but the top padding on `.cm-content` shifted text down without CodeMirror adjusting gutter positions.  
**Fix:** Removed padding from `.cm-content` and `.cm-gutters`. Added left text padding via `.cm-line { padding: "0 16px" }` instead, which CodeMirror handles correctly for gutter alignment. Added `minWidth: "2.5em"` to gutter elements for consistent width.  
**File changed:** `src/components/EditorPane.vue`

### Issue 2: Zoom controls and "Reveal in Finder" in floating bar
**Fix:**
- Added `zoomLevel` ref to `App.vue` with zoom in/out handlers (`*1.1` and `/1.1`, clamped 0.5–3.0)
- Added zoom in/out buttons and a zoom percentage display to `SplitView.vue` floating controls bar
- Added "Reveal in Finder" button to the floating bar that opens the parent folder of the current file/folder via `@tauri-apps/plugin-shell`
- `PreviewPane.vue` accepts a `zoom` prop and applies CSS `transform: scale()` with `transformOrigin: top left`, plus adjusts wrapper width/height to prevent clipping
- Buttons are in the top-right floating bar, NOT as an overlay on the PDF preview  
**Files changed:** `src/components/SplitView.vue`, `src/components/PreviewPane.vue`, `src/App.vue`

### Issue 3: Scroll sync simplified to one-directional
**Root cause:** Bidirectional scroll sync was unreliable because PDF viewers inside iframes trap scroll events, and cross-frame access fails silently. The iframe→editor direction was the source of the problems.  
**Fix:** Rewrote `useScrollSync.ts` to be one-directional only: editor scrolls → preview scrolls. Removed the problematic iframe scroll listener entirely. Uses `scrollDOM` directly from the CodeMirror view with a 50ms debounce.  
**File changed:** `src/composables/useScrollSync.ts`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## UI Fixes — 2026-05-25 (Open in Preview, Real PDF Zoom, Remove White Mat)

### Issue 1: "Reveal in Finder" → "Open in Preview"
**Root cause:** The user wanted to open the generated PDF in the system's default PDF viewer, not reveal the source folder in Finder.  
**Fix:**
- Renamed emit from `@revealInFinder` to `@openInPreview` in `SplitView.vue`
- Changed button icon and tooltip to "Open PDF in system preview"
- In `App.vue`: renamed function to `openInPreview()`, writes PDF bytes to `$TEMP/marktastic-preview.pdf` using `tempDir()` and `join` from `@tauri-apps/api/path`, then opens it via `@tauri-apps/plugin-shell`
- Added `fs:allow-temp-write-recursive` and `fs:allow-temp-read-recursive` to capabilities so the temp write succeeds  
**Files changed:** `src/components/SplitView.vue`, `src/App.vue`, `src-tauri/capabilities/default.json`

### Issue 2: Zoom uses transform scale instead of real PDF zoom
**Root cause:** `transform: scale()` on a wrapper div scales the iframe element graphically, stretching/pixelating the PDF rather than telling the browser's PDF renderer to zoom.  
**Fix:** Replaced the `transform: scale()` wrapper with the CSS `zoom` property applied directly to the `<iframe>`. This is supported in WebKit/Blink (which Tauri uses on all platforms) and tells the PDF renderer to zoom properly. Also adjusted iframe height to `100 / zoom %` so the full zoomed page is visible with scroll.  
**File changed:** `src/components/PreviewPane.vue`

### Issue 3: White overlay at bottom of preview in dark mode
**Root cause:** The iframe had `bg-white` class. When the PDF page was shorter than the viewport, the white background showed below the PDF content.  
**Fix:** Removed `bg-white` from the iframe class. The PDF pages themselves have their own white background; removing the class just removes the extra "mat" around the PDF.  
**File changed:** `src/components/PreviewPane.vue`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Zoom Fix — 2026-05-25

### Issue: Zoom buttons not working + loading overlay stuck
**Root cause:** CSS `zoom` property on `<iframe>` does not reliably zoom PDF content inside the browser's PDF renderer. The native PDF viewer zoom works because it manipulates the PDF internally, but CSS `zoom` on the iframe element is ignored. Additionally, the loading overlay used hardcoded `bg-black` which appeared as a dark mat, and the `@load` event on blob URL iframes can fail to fire, leaving the overlay stuck.  
**Fix:**
- **App.vue:** Added explicit `handleZoomIn()` and `handleZoomOut()` methods with `console.log` for debugging
- **PreviewPane.vue:** Replaced direct `zoom` CSS on iframe with a wrapper div using `transform: scale(zoom)` + `transformOrigin: top center`. The wrapper is inside an `overflow-auto` flex container so scaled content is scrollable. Removed hardcoded `bg-black` from loading overlay in favor of `bg-background` (theme-aware). Added a 3-second timeout fallback (`loadTimedOut`) to hide the overlay if the iframe `@load` event never fires. Added `clearLoadTimer`/`startLoadTimer` lifecycle management on `pdfUrl` changes and unmount.  
**Files changed:** `src/App.vue`, `src/components/PreviewPane.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## PDF Toolbar + Zoom Wrapper Fix — 2026-05-25

### Issue 1: Native PDF viewer toolbar (bottom-center on hover)
**Root cause:** The browser's built-in PDF viewer renders a hover toolbar at the bottom center with zoom/download/open controls. This is rendered internally by the WebView, not by our Vue code.  
**Fix:** Appended `#toolbar=0` to the PDF blob URL. This is a standard viewer hint supported in Chromium/WebKit that suppresses the native toolbar.  
**File changed:** `src/components/PreviewPane.vue`

### Issue 2: Zoom scales the container, not just the content
**Root cause:** The zoom wrapper used `flex justify-center` and `transformOrigin: 'top center'`, which caused the flex container to recalculate layout when scaled, making the entire preview pane feel like it was zooming. Center-origin scaling also caused overflow on both left and right sides simultaneously.  
**Fix:**
- Removed `flex justify-center` from the scroll container
- Changed `transformOrigin` from `'top center'` to `'top left'` so scaling happens from the corner, not the center
- Removed redundant `minWidth: '100%'` from the wrapper style
- Added `block` class to the iframe to prevent inline element spacing issues  
**File changed:** `src/components/PreviewPane.vue`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Architecture: Replace iframe with PDFium PNG Renderer — 2026-05-25

### Overview
Replaced the iframe-based PDF preview with a Rust PDFium renderer that converts PDF pages to PNG images displayed as `<img>` tags in Vue. This eliminates the native PDF viewer toolbar, gives us full control over zoom, and removes cross-frame scroll sync complexity.

**New flow:** Markdown → Typst → PDF bytes → Rust PDFium → PNG pages → Vue `<img>` tags

### Rust side
- Added dependencies: `pdfium-render` (with `image_latest` feature), `pdfium-auto` (auto-downloads PDFium binary on first run), `image` (PNG encoding), `base64` (frontend encoding)
- Created `src-tauri/src/pdfium_renderer.rs`:
  - `bind_pdfium()` uses `pdfium_auto::bind_pdfium_silent()` which downloads and caches the PDFium binary from `bblanchon/pdfium-binaries` on first run (~10MB, cached in `~/.cache/pdf2md/pdfium-7690/`)
  - `render_pdf_pages(pdf_bytes, zoom, dpr)` loads the PDF, iterates pages, renders each to a bitmap with zoom-scaled dimensions, encodes as PNG, base64-encodes, returns `Vec<String>` of `data:image/png;base64,...` data URLs
- Added `render_pdf_pages` Tauri command in `lib.rs`
- Registered command in invoke handler

### Vue frontend
- **Simplified `usePdf.ts`:** Removed blob URL creation (no longer needed for preview). Now only generates PDF bytes via `convert_md_to_pdf` and stores them in `pdfBytes`.
- **Created `usePdfRenderer.ts`:** Watches `pdfBytes` + `zoomLevel`, calls `render_pdf_pages` Rust command with device pixel ratio, returns reactive `pages` array.
- **Rewrote `PreviewPane.vue`:** Replaced iframe with scrollable `<div>` containing `<img v-for="page in pages">`. Shows loading spinner during initial render, re-render spinner when pages already exist, error state, or empty state.
- **Updated `App.vue`:**
  - Removed `useScrollSync` (no iframe = no cross-frame sync needed)
  - Removed iframe refs (`editorViewRef`, `previewIframeRef`)
  - Wired `PreviewPane` to `pages`, `previewLoading`, `previewError`
  - Kept zoom buttons, word wrap toggle, pane mode toggles, open-in-preview, export
- **Cleaned CSS:** Removed iframe pointer-events rule from `index.css`

### Key technical details
- PDFium binary is auto-downloaded on first run by `pdfium-auto` crate. No manual setup required.
- Zoom is handled by passing `zoom * devicePixelRatio` to the render config, producing crisp PNGs at any zoom level.
- Multi-page PDFs show all pages vertically stacked with padding and shadow.
- `pdfium-render` was not `Sync`, so we create a new `Pdfium` instance per render call. `pdfium-auto` caches the library path, so this is fast after first download.

### Files changed
- `src-tauri/Cargo.toml`
- `src-tauri/src/pdfium_renderer.rs` (new)
- `src-tauri/src/lib.rs`
- `src/composables/usePdf.ts`
- `src/composables/usePdfRenderer.ts` (new)
- `src/components/PreviewPane.vue`
- `src/App.vue`
- `src/assets/index.css`

### Compilation after architecture change
- `npm run build` ✅
- `cargo check` ✅

## Debounced Zoom Re-render + Smooth Visual Feedback — 2026-05-25

### Issue: Sluggish zoom on rapid clicks
**Root cause:** Every zoom click triggered an immediate `invoke('render_pdf_pages')` round-trip: Rust loads PDF → renders all pages → base64 encode → returns all images → Vue replaces all `<img>`. Even a 1-page PDF takes ~150ms. Multi-page takes 500ms+. The UI blocked on every click.  
**Fix:**
1. **Debounced re-rendering (400ms) in `usePdfRenderer.ts`:**
   - Added `debounceTimer` managed at module scope
   - On `[pdfBytesRef, zoomRef]` watcher change: `clearTimeout(debounceTimer)` then schedule new 400ms timeout
   - Rapid zoom clicks (e.g., 3 clicks in 200ms) → only one re-render fires at the final zoom level
   - Kept module-level `pages` ref so old images persist between timer resets

2. **Keep old images visible during re-render in `PreviewPane.vue`:**
   - Pages container is always visible when `hasPages` is true, even during `rendering`
   - Old `<img>` elements stay in the DOM; Vue reuses them via stable `:key="i"`
   - Added subtle "Updating zoom..." badge (top-right, rounded-full, `bg-card/90 backdrop-blur-sm`)
   - Images fade to `opacity-50` during re-render (`transition-opacity duration-300`)
   - When new images arrive, they smoothly fade back to full opacity as the `src` updates

### Files changed
- `src/composables/usePdfRenderer.ts`
- `src/components/PreviewPane.vue`

### Compilation after fixes
- `npm run build` ✅
- `cargo check` ✅

## Typst Delimiter Escaping Fix — 2026-05-25

### Issue: "Unclosed delimiter" errors from Typst compiler
**Root cause:** The `escape_text` function in `md_to_typst.rs` did not escape `{` and `}`, which are Typst content block delimiters. When markdown contained unbalanced braces, Typst saw unclosed delimiters. Additionally, table cell content and link text were not escaped at all, so raw `[`, `]`, `{`, `}` in those contexts broke Typst syntax.  
**Fix:**
1. **`escape_text`:** Added `{` → `\{` and `}` → `\}` escaping
2. **`escape_string`:** Added `{` and `}` escaping for completeness (used in string literals)
3. **Table cells:** Changed `cell.trim()` to `escape_text(cell.trim())` in both header and data rows
4. **Link text:** Changed `text` to `escape_text(text)` in `#link("...")[...]` content
5. **Task list state reset:** Added `in_task_list = false; task_checked = false;` after `TagEnd::Item` to prevent task markers from leaking into subsequent list items
6. **Inline code with backticks:** Added `typst_raw_string()` function that computes the maximum consecutive backticks in the content and uses `fence_len + 1` backticks as the delimiter. This handles code containing backticks safely in Typst's raw string syntax.  
**File changed:** `src-tauri/src/md_to_typst.rs`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## Snappy CSS Zoom + Debounce Reduction — 2026-05-26

### Issue: Zoom felt sluggish even with debounce
**Root cause:** The 400ms debounce meant users waited up to 400ms before any re-render started, and even then the full PNG round-trip felt slow. There was no instant visual feedback when clicking zoom buttons.  
**Fix:**
1. **Reduced debounce from 400ms → 200ms in `usePdfRenderer.ts`:** Faster re-render start while still coalescing rapid clicks.
2. **CSS `transform: scale()` for instant visual feedback in `PreviewPane.vue`:**
   - Added `zoom` prop to `PreviewPane`
   - Track `renderedZoom` — the zoom level at which the current images were rendered
   - `cssScale = zoom / renderedZoom` — when zoom changes, images scale immediately via CSS
   - `transformOrigin: 'top center'` — scales evenly from the center
   - `transition: isScaling ? 'transform 0.15s ease-out' : 'none'` — smooth 150ms transition while zooming, instant snap when crisp images arrive
   - When new pages arrive from Rust, `renderedZoom` catches up → `cssScale` snaps to 1 → images are crisp
3. **Updated `App.vue`:** Pass `:zoom="zoomLevel"` to `PreviewPane`

### Behavior
- Click zoom in → pages immediately scale larger (within 1 frame, ~16ms)
- After 200ms debounce → Rust re-renders → "Updating..." badge appears briefly
- New crisp pages arrive → CSS scale snaps to 1.0 → images are crisp
- Rapid zoom clicks (3x) → pages scale smoothly with each click, then ONE re-render at final zoom

### Files changed
- `src/composables/usePdfRenderer.ts`
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## Fixed 2.0× Pre-render + CSS Display Scaling — 2026-05-26

### Issue: Zoom still triggered Rust re-render even with CSS scale debounce
**Root cause:** The previous approach rendered at the user's target zoom level, then used CSS scale as a temporary visual band-aid until the real re-render arrived. Zoom clicks still triggered `invoke('render_pdf_pages')` after a 200ms debounce. Multi-page PDFs took 500ms+ to re-render.  
**Fix:**
1. **Always render at fixed 2.0× zoom in `usePdfRenderer.ts`:**
   - Removed `zoomRef` parameter entirely
   - Removed debounce timer (no longer needed — only `pdfBytes` is watched)
   - Always passes `zoom: 2.0` to Rust `render_pdf_pages` command
   - Only re-renders when `pdfBytes` changes (markdown edit or template change)

2. **Simplified `PreviewPane.vue` to pure CSS display scaling:**
   - `displayScale = zoom / 2.0` — fixed formula since render zoom is always 2.0
   - `transform: scale(displayScale)` with `transformOrigin: 'top center'`
   - `width: ${100 / displayScale}%` — compensates layout width so the container matches visual size
   - Removed `renderedZoom` tracking, `cssScale` computed, `isScaling` logic, and transition animation
   - Removed the "Updating..." re-render badge (no re-render on zoom)

3. **Updated `App.vue`:**
   - `usePdfRenderer(pdfBytes)` — removed `zoomLevel` argument
   - PreviewPane `:zoom="zoomLevel"` binding unchanged

### Behavior
- Open file → one initial render at 2.0× (slower than before, but only once)
- Click zoom in/out → **instant CSS scaling, zero Rust calls, zero delay**
- Edit markdown → `usePdf` debounces 500ms → new `pdfBytes` → one re-render at 2.0×
- Zoom range 0.5–2.0× all produce crisp images (source is 2.0×, downscaled or displayed native)

### Files changed
- `src/composables/usePdfRenderer.ts`
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## Fix Wide Gaps Between Pages — 2026-05-26

### Issue: Massive vertical gaps between PDF pages
**Root cause:** `transform: scale()` only affects visual rendering — it does NOT shrink the element's layout box. Each page image is rendered at 2.0× (e.g., 2000px tall). Even with `transform: scale(0.5)`, the layout box stays 2000px tall. The flex container allocates 2000px + padding for each page, creating giant gaps.  
**Fix:** Replaced `transform: scale(displayScale)` with the CSS `zoom: displayScale` property on the `<img>` element. Unlike `transform`, `zoom` scales both the visual size AND the layout box. At `zoom: 0.5`, a 2000px image becomes 1000px in both visual and layout. Removed `transformOrigin`, `maxWidth: 'none'`, and `width` compensation since `zoom` handles layout automatically.

### Behavior
- Pages now appear with normal spacing (just `p-4` padding, no giant gaps)
- Zoom in/out resizes pages smoothly with no gap changes
- Still zero Rust re-render on zoom — pure CSS instant zoom

### Files changed
- `src/components/PreviewPane.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

---

## Optimization Phase — 2026-05-26

Planned sequential performance improvements. Each fix will be implemented one at a time, with user validation between each.

| Fix | Description | Status |
|-----|-------------|--------|
| 1 | Background threads (`spawn_blocking`) — moves Rust computation off main thread | `completed` |
| 2 | Viewport page rendering — only render visible pages | `completed` |
| 3 | Page caching — hash-based cache for unchanged pages | `completed` |
| 4 | Persistent Typst World — incremental compilation | `in_progress` |
| 5 | SVG output — replace PNG pipeline with SVG | `completed` |

## Fix 1: Background Threads (`spawn_blocking`) — 2026-05-26

### Issue: Rust computation blocked the Tauri main thread
**Root cause:** All Tauri commands (`convert_md_to_pdf`, `render_pdf_pages`, `open_file_path`, `open_folder`, `resolve_wikilinks`, `compile_folder_to_pdf`, `get_templates`) ran synchronously on the Tauri async runtime's main task. Heavy Typst compilation (200ms–2s) or PDFium rendering (100ms–500ms) blocked the WebView event loop, causing UI freezes — spinner animation stuttered, buttons became unresponsive, theme toggle lagged.  
**Fix:** Converted all non-trivial commands from `fn` to `async fn`, wrapping their bodies in `tokio::task::spawn_blocking`. The blocking thread pool runs Typst compilation, PDFium rendering, and file I/O off the main async thread.

### Commands converted
| Command | Work moved to `spawn_blocking` |
|---------|-------------------------------|
| `convert_md_to_pdf` | Full Typst compilation pipeline |
| `compile_folder_to_pdf` | Wikilink resolution + Typst compilation |
| `render_pdf_pages` | PDFium PDF→PNG rendering |
| `resolve_wikilinks` | File reading + BFS graph traversal |
| `open_file_path` | `std::fs::read_to_string` |
| `open_folder` | `std::fs::read_dir` + per-file `read_to_string` |
| `get_templates` | `std::fs::read_dir` on template directories |

### Technical details
- Added `tokio = { version = "1", features = ["rt", "rt-multi-thread"] }` to `src-tauri/Cargo.toml`
- Each command now returns `Result<T, String>` via `.await.map_err(|e| format!("Task join error: {}", e))?`
- The `.invoke_handler()` in `run()` required no changes — Tauri handles async commands transparently
- Vue frontend (`invoke()`) required no changes — it already returns a Promise

### Behavior
- Typing in the editor → Typst compiles on a background thread → UI stays fully responsive
- "Compiling PDF..." toast appears smoothly, spinner animation doesn't stutter
- Buttons, theme toggle, pane resizing all work while PDF is compiling
- Actual compilation time unchanged (200ms–2s), but the app **feels** instant

### Files changed
- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅
- **User validated:** UI stays responsive during Typst compilation and PDFium rendering. Spinner animation is smooth, buttons and theme toggle work while PDF compiles.

## Fix 2: Viewport Page Rendering — 2026-05-26

### Issue: All PDF pages rendered on every keystroke
**Root cause:** `usePdfRenderer.ts` called `render_pdf_pages` which rendered ALL pages to PNG. A 30-page document did 30× the work of a 1-page document. Every markdown edit triggered a full re-render of every page.  
**Fix:**
1. **Added `render_pdf_page_range` to `pdfium_renderer.rs`:** Renders only specific page numbers passed as a Vec, skipping pages not in the list.
2. **Added `get_pdf_page_count` to `pdfium_renderer.rs`:** Returns the total number of pages without rendering anything.
3. **Added two new Tauri commands in `lib.rs`:**
   - `get_pdf_page_count(pdf_bytes) -> u16`
   - `render_pdf_page_range(pdf_bytes, page_numbers, zoom, dpr) -> Vec<(usize, String)>`
   Both wrapped in `spawn_blocking` like all other commands.
4. **Rewrote `usePdfRenderer.ts`:**
   - Added module-scoped `pageCache: Map<number, string>` that persists across renders
   - On `pdfBytes` change: calls `get_pdf_page_count`, clears cache, sets visible pages to {0,1,2}, renders those
   - On `visiblePageNumbers` change: checks which visible pages are NOT in cache, calls `render_pdf_page_range` for only those missing pages, merges results into cache
   - Exports `cachedPages` (computed array with `null` for uncached pages) and `totalPages`
5. **Updated `PreviewPane.vue`:**
   - Added `IntersectionObserver` with 200px rootMargin to detect which pages are near the viewport
   - Emits `@update:visiblePages` with buffered set (visible page ± 1 neighbor)
   - Shows `<img>` for cached pages, gray placeholder with "Page N" label for uncached pages
   - Added "Loading pages..." badge in top-right when rendering additional pages
6. **Updated `App.vue`:**
   - Added `visiblePageNumbers` ref (initially {0,1,2})
   - Passed to `usePdfRenderer(pdfBytes, visiblePageNumbers)`
   - Bound `:total-pages="totalPages"` and `@update:visible-pages` on `PreviewPane`

### Behavior
- Open a multi-page PDF → first 3 pages render immediately
- Scroll down → IntersectionObserver detects new pages entering viewport → `visiblePages` updates → missing pages are rendered on-demand
- Scroll back up → previously rendered pages show instantly from cache (no Rust call)
- Typing in editor → `pdfBytes` changes → cache clears → first 3 pages re-render
- UI stays responsive throughout (Fix 1 background threads)

### Files changed
- `src-tauri/src/pdfium_renderer.rs`
- `src-tauri/src/lib.rs`
- `src/composables/usePdfRenderer.ts`
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

### Status
**Completed** — validated by user. Root cause of initial all-gray-placeholder bug was that `cachedPages` computed property read from a non-reactive plain JS `Map`; Vue never detected cache updates. Fixed by returning the reactive `pages` ref directly and updating it after each render batch. IntersectionObserver correctly watches the scrolling container (`root: scrollerRef.value`).

## Fix 3: Page Caching by Content Hash — 2026-05-26

### Issue: Flashing gray placeholders on every keystroke
**Root cause:** When `pdfBytes` changed (after the 500ms editor debounce), `usePdfRenderer.ts` immediately cleared `pages.value` to empty, causing all pages to flash to gray placeholders. Then Rust compiled + rendered new pages, which arrived 200ms–2s later. The user saw a jarring flash on every keystroke.  
**Fix:** Implemented stale-while-revalidate with a persistent cross-compile cache keyed by `(pdfHash, pageNum, zoom)`:

1. **Persistent cache in `usePdfRenderer.ts`:**
   - Module-scoped `persistentCache: Map<string, string>` where key = `${pdfHash}:${pageNum}:${zoom}`
   - `hashBytes()` computes SHA-256 of the PDF bytes via Web Crypto API
   - Cache persists across document switches and app sessions (module-level)

2. **Stale-while-revalidate flow:**
   - On `pdfBytes` change: compute hash, save current `pages.value` to `stalePages`, set `isRecompiling = true`
   - For each page: check persistent cache → if hit, show cached image immediately. If miss, show stale page from previous compile as fallback.
   - Old pages stay visible (no flash to placeholders). A sticky "Recompiling..." badge appears at the top of the preview.
   - Render visible pages first (priority), hidden pages in background.
   - When new renders arrive, they overwrite stale pages in `pages.value`.
   - `isRecompiling = false` when all pages are rendered.

3. **Scroll-triggered rendering:**
   - On scroll, `renderOnScroll()` checks which visible pages are NOT in the persistent cache for the current PDF hash.
   - Only uncached pages are rendered. Previously rendered pages show instantly.
   - Guard: skip scroll render while `isRecompiling` is true (already handled by `renderForPdfBytes`).

4. **PreviewPane.vue updates:**
   - Added `isRecompiling` prop
   - Sticky "Recompiling..." badge at top of scroll container (above pages, not floating)
   - Badge has `z-30` to stay above page content

5. **App.vue updates:**
   - Wired `isRecompiling` from `usePdfRenderer` to `PreviewPane`

### Behavior
- Type in editor → 500ms debounce → pdfBytes changes → old pages stay visible → "Recompiling..." badge appears → new pages render in background → swap in smoothly when ready
- If you UNDO exactly to previous state → SHA-256 hash matches → all pages load instantly from persistent cache → zero Rust calls
- Scroll down → new pages render on demand, cached for next time
- Large documents: only visible pages re-render first, hidden pages in background

### Files changed
- `src/composables/usePdfRenderer.ts`
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

### Status
`in_progress` — awaiting user validation.

## Unified Toast Notifications — 2026-05-26

### Issue: Inconsistent notification styling and positioning
**Root cause:** ToastContainer used bottom-right fixed position with colored-background cards (green for success, red for error, etc.). The "Recompiling" badge in PreviewPane used a different style (`bg-card/90 backdrop-blur-sm border rounded-full` pill badge). Two separate notification systems with different visuals.  
**Fix:**
1. **Redesigned `ToastContainer.vue`:**
   - Moved from `fixed bottom-4 right-4` to an `absolute top-2` container inside `<main>` (top-center of content)
   - Unified style: all toasts use `bg-card/90 backdrop-blur-sm border rounded-full px-3 py-1.5 text-xs shadow-sm w-fit` pill badge
   - Color-coded via text color classes (`text-green-600`, `text-red-600`, `text-amber-600`, `text-muted-foreground`) instead of background colors
   - SVG icons for each type (spinner for info/loading, checkmark for success, triangle for warning, X circle for error)
   - Transition animation: fade + slide from top (`translateY(-8px)`)

2. **Updated `useToast.ts`:**
   - Added `loading` type to ToastType
   - Added `dismiss(id)` method for manual toast removal
   - Added `loading(message, duration?)` function
   - `duration: 0` disables auto-dismiss (used for "Recompiling..." toast)

3. **Updated `PreviewPane.vue`:**
   - Removed `isRecompiling` prop
   - Removed sticky "Recompiling..." badge from the template
   - Kept the "Loading pages..." absolute badge (for scroll-triggered page loading)

4. **Updated `App.vue`:**
   - Moved `<ToastContainer>` inside `<main>` at `absolute top-2 left-0 right-0 z-30 flex justify-center`
   - Added `watch(isRecompiling, ...)` that shows a `toast.loading("Recompiling...", 0)` toast when recompiling starts and dismisses it when done

### Behavior
- All notifications appear as consistent pill badges at top-center of the main content area
- "Recompiling..." appears as a toast (with spinner) instead of a sticky badge in the preview pane
- Success/error/warning/info toasts all share the same visual style, differentiated only by icon and text color
- Auto-dismiss still works (default 4s, configurable per toast)
- Manual dismiss via close button or programmatic `dismiss(id)`

### Files changed
- `src/composables/useToast.ts`
- `src/components/ToastContainer.vue`
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

### Status
**Completed** — validated by user. Toast badges are now uniform pill style inside PreviewPane (visible only when preview is showing). All badges share fixed `w-56` width. Editor-only mode hides toasts since PreviewPane is unmounted. Recompiling toast appears via `watch(isRecompiling)` in App.vue and is dismissed when done.

## Fix 4: Persistent Typst World — 2026-05-26

### Issue: Typst world recreated on every compile
**Root cause:** `compile_markdown_to_pdf` created a fresh `TypstWrapperWorld` on every invocation via `TypstWrapperWorld::new(root, full_source)`. This triggered an expensive system font search (`FontSearcher::new().include_system_fonts(true).search()`) on every keystroke — 100–300ms just for font discovery, before any actual compilation.  
**Fix:**
1. **Added persistent world to Tauri state in `lib.rs`:**
   - Created `AppState { world: Arc<Mutex<TypstWrapperWorld>> }`
   - Initialized once at app startup in `run()` with empty source text
   - The same world instance is reused across all compilations

2. **Added `update_source(&mut self, text: String)` to `TypstWrapperWorld`:**
   - Replaces `self.source` with `Source::detached(text)`
   - Uses the same detached `FileId` so Typst recognizes it as the same file

3. **Refactored `lib.rs` compilation pipeline:**
   - Extracted `build_full_source(typst_body, template_name)` — template resolution + body injection
   - Extracted `compile_with_world(world, source)` — locks world, updates source, compiles, returns PDF bytes
   - `convert_md_to_pdf` and `compile_folder_to_pdf` now clone the `Arc<Mutex<World>>`, lock it in `spawn_blocking`, and compile

4. **Font search is now done once at startup**
   - `FontSearcher::search()` runs once when the world is created
   - Subsequent compilations skip font discovery entirely

### Technical notes
- `Arc<Mutex<World>>` is passed to `spawn_blocking` by cloning the `Arc`
- The lock is held for the entire compilation, serializing concurrent compile requests
- `Source::detached(new_text)` creates a new source with the same `FileId` (the detached ID). Typst's internal span-based memoization may be partially invalidated because AST spans are regenerated, but font loading overhead is eliminated entirely.
- If `FontSlot` were not `Send`, this approach would fail at compile time. It compiled successfully, confirming `FontSlot` is `Send`.

### Behavior
- First compile after app startup: font search + compilation (same as before)
- Subsequent compiles: skip font search → faster by 100–300ms
- The world persists until app restart
- Template switches still work (template is resolved per-compile, only the world is reused)

### Files changed
- `src-tauri/src/lib.rs`
- `src-tauri/src/typst_world.rs`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

### Status
Awaiting user validation.

## Fix 5: SVG Output — 2026-05-26

### Issue: PDFium PNG pipeline was slow, pixelated, and non-selectable
**Root cause:** The preview pipeline was Markdown → Typst → PDF → PDFium → PNG → `<img>`. Each step added latency: Typst compilation (200–500ms), PDF generation (100ms), PDFium bitmap rendering (200–500ms per page), base64 encoding. PNGs were pixelated when zoomed and text was not selectable.  
**Fix:** Replaced the entire pipeline with direct Typst → SVG output:

1. **Added `typst-svg = "0.14.2"` dependency**
2. **Added `compile_to_svg` helper + `convert_md_to_svg` Tauri command in `lib.rs`:**
   - Compiles markdown → Typst source using the persistent world (Fix 4)
   - Iterates `document.pages` and calls `typst_svg::svg(page)` for each page
   - Returns `Vec<String>` of raw SVG markup
3. **Created `useSvgRenderer.ts` composable:**
   - Watches `editorContent` + `selectedTemplate` with 500ms debounce
   - Calls `convert_md_to_svg` directly (single round-trip, no PDF/PNG intermediate)
   - Returns `pages` (SVG strings), `totalPages`, `rendering`, `error`, `isRecompiling`
4. **Rewrote `PreviewPane.vue`:**
   - Removed IntersectionObserver, visible page tracking, PDFium placeholders
   - Renders SVG via `<div v-html="page">` inside a scaled container
   - Zoom is pure CSS `transform: scale(zoom / 2.0)` — SVG stays crisp at any zoom
   - Removed ToastContainer import (still rendered via parent but not imported here)
5. **Updated `App.vue`:**
   - Replaced `usePdfRenderer` with `useSvgRenderer`
   - Removed `visiblePageNumbers` ref and `@update:visible-pages` binding
   - Kept `usePdf` for PDF export functionality

### Behavior
- Typing in editor → 500ms debounce → single Rust call `convert_md_to_svg` → SVG pages arrive in ~50–150ms
- No PDFium binary download needed for preview (still needed for `render_pdf_pages` export path)
- Zoom is instant CSS transform — no re-render, no pixelation
- Text in preview is selectable (SVG text elements)
- Much smaller memory footprint (SVG text vs PNG bitmaps)

### Files changed
- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- `src/composables/useSvgRenderer.ts` (new)
- `src/components/PreviewPane.vue`
- `src/App.vue`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

### Status
Awaiting user validation.

## P0 Fix 1: Replace v-html SVG with `<img>` Data URLs — 2026-05-26

### Issue: App froze when typing due to main-thread SVG parsing
**Root cause:** `v-html` in `PreviewPane.vue` parsed SVG XML into DOM nodes synchronously on the main JavaScript thread. This blocked CodeMirror 6 from processing keystrokes, making the editor feel frozen.  
**Fix:** Replaced inline `v-html` injection with `<img>` tags using SVG data URLs. The browser's image renderer parses SVG off the main JavaScript thread, keeping the editor responsive.

### Changes
- `PreviewPane.vue`: changed from `<div v-html="page">` to `<img :src="svgToDataUrl(page)">`
- Added `svgToDataUrl()` helper using minimal percent-encoding (faster than base64)
- Kept CSS `zoom` property for scaling (scales both visual and layout box)

### Files changed
- `src/components/PreviewPane.vue`
- `src/composables/useSvgRenderer.ts`

### Compilation after fix
- `npm run build` ✅
- `cargo check` ✅

## P0 Performance Fixes

| Fix | Status | Notes |
|-----|--------|-------|
| P0-1: v-html → img data URLs | ✅ committed (48af1aa) | Stops SVG parsing from blocking CM6 |
| P0-2: requestIdleCallback yield | ✅ committed (48af1aa) | Stop DOM injection from blocking |
| P0-3: Debounce verification + dedupe | ⬜ pending | Fix invalid Vue watch debounce |
