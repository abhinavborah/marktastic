# Marktastic — Handoff

## What Has Been Done
- Initial setup commit (git repo initialized at `~/Developer/marktastic`)
- Phase 1 (Project Scaffolding) completed — Tauri v2 + Vue 3 + TypeScript app scaffolded
- Tailwind CSS v3 + PostCSS configured
- shadcn-vue manually installed (`cn()` utility, CSS variables, animations)
- CodeMirror 6 packages installed and ready
- Tauri v2 plugins (`dialog`, `fs`, `shell`) installed — npm + Rust deps + `lib.rs` initializers + capabilities permissions configured
- `src-tauri/templates/` directory created for built-in Typst templates
- Compilation verified: `npm install`, `npm run build`, `cargo check`, and `tauri dev` all passed
- Context files written: `context/task_plan.md`, `context/findings.md`, `context/progress.md`
- `handoff.md` and `context/` directory regenerated after accidental deletion during scaffolding
- Phase 2 (Rust Backend — Typst Pipeline) completed:
  - Rust dependencies added: `pulldown-cmark`, `typst`, `typst-pdf`, `typst-kit`, `comemo`, `time`, `ureq`, `tar`, `zune-inflate`, `ttf-parser`
  - `src/typst_world.rs` — custom `TypstWrapperWorld` implementing `typst::World` trait
  - `src/md_to_typst.rs` — Markdown→Typst converter with GFM support (tables, lists, strikethrough, code blocks, links, images, blockquotes)
  - 3 built-in Typst templates: `basic-report.typ`, `university-assignment.typ`, `thesis-chapter.typ`
  - Tauri commands implemented:
    - `convert_md_to_pdf(markdown, template_name) -> Result<Vec<u8>, String>` — full MD→Typst→PDF in-memory pipeline
    - `open_file_path(file_path) -> Result<String, String>` — read `.md` file contents
    - `open_folder(folder_path) -> Result<Vec<(String, String)>, String>` — list `.md` files in folder
    - `get_templates() -> Result<Vec<String>, String>` — return available template names
  - All commands registered in `.invoke_handler()`
  - `cargo check` passed with 0 errors, 0 warnings
- Phase 3 (Vue Frontend) completed:
  - `src/types.ts`, `src/composables/useTheme.ts`, `src/composables/usePdf.ts`
  - `WelcomeScreen.vue` — landing screen with Open File / Open Folder buttons
  - `EditorPane.vue` — CodeMirror 6 with markdown syntax highlighting, line numbers, light/dark theme switching
  - `PreviewPane.vue` — PDF iframe with loading, error, and empty states
  - `SplitView.vue` — resizable horizontal split with drag handle and pane toggle buttons
  - `Toolbar.vue` — app header with template selector, theme toggle, export PDF, open file/folder
  - `App.vue` — main shell integrating all components, file/folder open flow via Tauri JS dialog, PDF export via save dialog + fs write
  - `npm run build` and `cargo check` both pass; `tauri dev` starts successfully
- Phase 4 (Wikilink Resolution + Merged Editor + Scroll Sync) completed:
  - `src/wikilinks.rs` — regex-based wikilink parser, BFS dependency resolver, merged document builder with section boundaries
  - `compile_folder_to_pdf(folder_path, template_name)` — resolves wikilinks, merges reachable files, compiles to PDF
  - `resolve_wikilinks(folder_path)` — returns ordered list of reachable `.md` files from entry point (`index.md` or `main.md`)
  - `preprocess_wikilinks()` in `md_to_typst.rs` — handles `[[...]]` syntax in single-file mode (converts to `*Missing link: ...*`)
  - `useScrollSync.ts` — percentage-based bidirectional scroll sync between CodeMirror editor and PDF iframe with loop prevention
  - `App.vue` updated for folder mode: merged editor view with `<!-- file: name -->` section headers, dynamic PDF command switching
  - `Toolbar.vue` updated with folder name badge and reachable file count
  - `WelcomeScreen.vue` updated with wikilink syntax documentation
  - `npm run build` and `cargo check` both pass
- Phase 5 (Dark Mode Polish, Keyboard Shortcuts, Toasts, UX Polish) completed:
  - `useToast.ts` + `ToastContainer.vue` — toast notifications with info/success/warning/error types, auto-dismiss, animations
  - `useKeyboard.ts` — platform-aware keyboard shortcuts (macOS Meta / others Ctrl): ⌘/Ctrl+O (open file), +Shift+O (open folder), +S (export PDF), +E/P/B (pane toggles), +T (cycle theme)
  - `usePdf.ts` integrated with toasts — "Compiling PDF...", "PDF ready", error messages
  - `App.vue` — toast integration, dynamic window title via `getCurrentWebviewWindow().setTitle()`, keyboard shortcut wiring
  - `PreviewPane.vue` — polished loading spinner, black overlay during iframe load to prevent white flash in dark mode
  - `Toolbar.vue` — macOS traffic light padding (`pl-20`), keyboard shortcut hints in tooltips and button labels
  - `SplitView.vue` — `v-model` pattern for pane mode, accessible via keyboard shortcuts
  - `src/assets/index.css` — smooth 0.3s theme transitions, custom styled scrollbars, focus-visible outline rings
  - `npm run build` and `cargo check` both pass

## 6-Phase Implementation Plan

| Phase | Focus | Status |
|-------|-------|--------|
| 1 | Project scaffolding (Tauri v2 + Vue 3 + shadcn-vue + CodeMirror 6 + Tailwind + capabilities) | Completed |
| 2 | Rust backend — Typst pipeline (`convert_md_to_pdf`, `open_file_path`, `open_folder`, templates) | Completed |
| 3 | Vue frontend — editor + live PDF preview + split view + file open/save/export | Completed |
| 4 | Wikilink resolution + GFM parsing + merged editor + scroll sync | Completed |
| 5 | Dark mode + polish (keyboard shortcuts, toasts, window controls, UX) | Completed |
| 6 | Build + GitHub release setup (cross-platform CI, binaries) | In Progress |

## Key Constraints
- Package manager: **npm** (not pnpm)
- Dark mode: follows OS preference on launch; toggleable after
- Window: native OS chrome, full screen default, resizable, min/max/close
- Markdown flavor: **GFM** (tables, task lists, strikethrough, autolinks)
- Templates: 3 built-in Typst templates (`basic-report`, `university-assignment`, `thesis-chapter`)
- Live preview: **PDF** (Typst-compiled) rendered in iframe, not HTML

## Context Files to Read
Before starting work, read:
- `context/task_plan.md` — full task plan and requirements
- `context/findings.md` — research findings and technical decisions
- `context/progress.md` — session log and current status

## Skills to Load
- `planning-with-files` — for structured planning
- `context7` — for Tauri v2 and Typst crate documentation lookups

## Next Actions (Phase 6)
1. Cross-platform build configuration (`tauri.conf.json` bundle settings for macOS, Windows, Linux)
2. GitHub Actions CI workflow for automated builds on push/tag
3. GitHub Releases setup with signed binaries
4. App code signing configuration (macOS notarization, Windows code signing if available)
5. Update `README.md` with installation and usage instructions
6. Tag v0.1.0 release

---

## Major Changes Since Last Handoff

### Architecture Change: iframe → Rust PDFium Renderer
The original architecture displayed PDFs via `<iframe src="blob:...">`. This was replaced with a Rust-based PDFium renderer due to fundamental limitations of the native iframe PDF viewer on macOS WKWebView:
- Native PDF viewer toolbar appears on hover (cannot be disabled via `#toolbar=0` or any other means on WKWebView)
- No programmatic zoom API — `transform: scale()` only stretches the rendered canvas
- Cross-frame scroll sync is unreliable

**New flow:** Markdown → Typst → PDF bytes → Rust PDFium → PNG pages → Vue `<img>` tags
- `src-tauri/src/pdfium_renderer.rs` — calls `pdfium_auto::bind_pdfium_silent()` to auto-download the PDFium binary on first run (~10 MB, cached), then renders pages to PNG at zoom-scaled resolution
- `src/composables/usePdfRenderer.ts` — watches `pdfBytes` + `zoomLevel`, invokes `render_pdf_pages` Rust command with 400 ms debounce
- `src/components/PreviewPane.vue` — scrollable `<div>` with `<img v-for="page in pages">`
- `useScrollSync.ts` is now **orphaned** (still exists in `src/composables/` but no longer imported by any component). The editor and preview scroll independently; no sync is attempted with the PNG-based preview

### Bugs Fixed (with root causes)

1. **Export to PDF fails ("fs write not allowed")**
   - Root cause: `fs:default` Tauri capability only grants read access; write to user directories requires explicit `fs:allow-{dir}-write-recursive`
   - Fix: Added `fs:allow-home-write-recursive`, `fs:allow-document-write-recursive`, `fs:allow-download-write-recursive`, `fs:allow-desktop-write-recursive` to `src-tauri/capabilities/default.json`
   - Also added `fs:allow-temp-write-recursive` and `fs:allow-temp-read-recursive` for the "Open in Preview" feature

2. **Folder-mode live preview doesn't reflect edits**
   - Root cause: `usePdf` previously called `compile_folder_to_pdf` on every edit, which re-read original disk files instead of using the edited merged content
   - Fix: `App.vue` `handleOpenFolder()` now builds the merged string once on open and stores it in `editorContent`; `usePdf` always calls `convert_md_to_pdf` with current editor content. The `compile_folder_to_pdf` command still exists in `lib.rs` but is no longer invoked from the frontend.

3. **Glitchy split pane resize + drag continues after mouse release**
   - Root cause: `document.addEventListener` for `mousemove`/`mouseup` doesn't capture events when the mouse is over an iframe or outside the window
   - Fix: Changed to `window.addEventListener` for `mousemove`, `mouseup`, and `blur` (handles Alt+Tab during drag). A `:data-dragging="isDragging"` attribute remains on the split container but no active CSS rule consumes it; the fix relies purely on `window`-scoped listeners.

4. **"Compiling PDF..." toast on app startup**
   - Root cause: `usePdf.ts` had `watch(..., { immediate: true })` which fired on mount with empty editor content
   - Fix: Added guard in the watch callback — if markdown is empty or whitespace-only, skip compilation and clear state

5. **Line numbers vertically misaligned**
   - Root cause: Vertical padding on `.cm-content` pushed text down without CodeMirror adjusting gutter numbers
   - Fix: Removed vertical padding from `.cm-content`; per-line horizontal padding moved to `.cm-line` (which CodeMirror handles correctly)

6. **Line numbers overlap scrolled text**
   - Root cause: `.cm-gutters` had `backgroundColor: "transparent"` in the CodeMirror theme
   - Fix: Added explicit `background-color: hsl(var(--muted) / 1) !important` for `.cm-gutters` in `src/assets/index.css`

7. **Zoom buttons laggy/sluggish**
   - Root cause: Every zoom click triggered an immediate full round-trip to Rust (PDFium load → render all pages → base64 → return)
   - Fix: 400 ms debounce on the `pdfBytes` + `zoomLevel` watcher in `usePdfRenderer.ts`; old pages stay visible during re-render with an opacity fade; an "Updating zoom..." badge appears in the top-right of the preview

8. **Typst "unclosed delimiter" errors on large markdown files**
   - Root cause: `escape_text` didn't escape `{` and `}`, which are Typst content block delimiters
   - Fix: Added `{` and `}` escaping to both `escape_text` and `escape_string`; also fixed table cells and link text to use `escape_text` consistently; added `typst_raw_string()` for inline code containing backticks

9. **Loading overlay stuck on preview** *(iframe-era issue, now moot)*
   - Root cause: iframe `@load` event sometimes failed to fire for blob URLs
   - Fix (at the time): Added 3-second timeout fallback (`loadTimedOut`) that hid the overlay if load never completed
   - **Current state:** With the PDFium renderer, loading state is driven by the `rendering` boolean from `usePdfRenderer.ts`; no timeout mechanism is needed

10. **White overlay at bottom of preview in dark mode** *(iframe-era issue, now moot)*
    - Root cause: iframe had a `bg-white` class; when the PDF page was shorter than the viewport, white showed below
    - Fix (at the time): Removed `bg-white` from the iframe
    - **Current state:** The PDFium renderer outputs `<img>` elements with `style="background-color: white;"`, which is correct because PDF pages are inherently white

11. **Bottom-center native PDF viewer toolbar visible** *(iframe-era issue)*
    - Root cause: `#toolbar=0` URL fragment is Adobe-specific, ignored by Apple's PDFKit in WKWebView
    - Fix: Eliminated the iframe entirely — replaced with the PDFium PNG renderer (no iframe = no native toolbar)

12. **"Reveal in Finder" → "Open in Preview"**
    - Original implementation used `@tauri-apps/plugin-shell` to open a folder in Finder
    - Changed to "Open in Preview" — writes the current PDF bytes to a temp file (`marktastic-preview.pdf` in the system temp directory) and opens it with the system default PDF app via `openInShell()`

13. **CSS `zoom` property doesn't work on iframe PDF**
    - Root cause: CSS `zoom` is non-standard and ignored by iframe PDF content
    - Fix: With the PDFium renderer, zoom is proper — pages are re-rendered at the correct resolution via `PdfRenderConfig` with scaled target width/height

### Current Component Layouts and Wiring

```
App.vue (root)
├── Toolbar (top header)
│   ├── Template selector dropdown
│   ├── Theme cycle toggle (☀/☾/◐)
│   ├── Export PDF button
│   └── Open File / Open Folder buttons
│
├── main area
│   ├── WelcomeScreen (when no file open)
│   │   ├── "Open File" button
│   │   └── "Open Folder" button
│   │
│   └── SplitView (editor + preview)
│       ├── EditorPane (left)
│       │   └── CodeMirror 6 (markdown, line numbers, word wrap)
│       │
│       ├── drag handle (1px vertical bar)
│       │
│       ├── PreviewPane (right)
│       │   └── Scrollable <img> list (PNG pages rendered by PDFium)
│       │
│       └── floating controls (top-right)
│           ├── Editor-only / Split / Preview-only buttons
│           ├── Word wrap toggle
│           ├── Zoom out / zoom percent / zoom in buttons
│           └── Open in Preview button
│
└── ToastContainer (bottom-right)
    └── Stacked toast notifications
```

### Data Flow

```
User opens file/folder
    ↓
Tauri dialog → Rust command (open_file_path / open_folder)
    ↓
editorContent (Vue ref)
    ↓
usePdf: watches editorContent + selectedTemplate → debounce 500ms → invoke('convert_md_to_pdf')
    ↓
Rust: md_to_typst → typst_world → typst_pdf → Vec<u8>
    ↓
pdfBytes (Vue ref)
    ↓
usePdfRenderer: watches pdfBytes + zoomLevel → debounce 400ms → invoke('render_pdf_pages')
    ↓
Rust: PDFium loads PDF → renders each page to bitmap → PNG → base64 data URL
    ↓
pages (Vue ref: string[] of data:image/png;base64, URLs)
    ↓
PreviewPane: v-for page in pages → <img :src="page">
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + O` | Open single file |
| `Ctrl/Cmd + Shift + O` | Open folder |
| `Ctrl/Cmd + S` | Export PDF |
| `Ctrl/Cmd + E` | Editor-only pane |
| `Ctrl/Cmd + P` | Preview-only pane |
| `Ctrl/Cmd + B` | Both panes |
| `Ctrl/Cmd + T` | Cycle theme (light → dark → system) |
| `Ctrl/Cmd + Shift + W` | Toggle word wrap |

### Known Limitations

1. **First PDFium load** — Binary downloads on first run (~10 MB) via `pdfium-auto`. Requires internet or a pre-bundled `libpdfium` binary set via `PDFIUM_LIB_PATH`.
2. **Zoom re-render latency** — Even with debounce, re-rendering multi-page PDFs at high zoom takes 200–500 ms. Old pages stay visible with an opacity fade during the update.
3. **No text selection in preview** — PNG images don't allow text selection. Would need OCR or a different rendering approach.
4. **No search in preview** — Same reason: the preview is a sequence of images, not text.
5. **File sidebar for folder mode** — Not implemented (would require tracking section boundaries in the editor for clickable navigation).
6. **Custom user-defined Typst templates** — Not yet supported. Templates are hardcoded in `src-tauri/templates/`.
7. **Print support** — Not implemented.
8. **Auto-save** — Not implemented.

### Tech Stack (Current)

| Layer | Technology |
|-------|------------|
| Backend | Rust + Tauri v2 |
| Frontend | Vue 3 + Vite + TypeScript |
| UI | shadcn-vue + Tailwind CSS |
| Editor | CodeMirror 6 |
| PDF Generation | Typst (`typst`, `typst-pdf`, `typst-kit` crates) |
| PDF Rendering | Google PDFium (`pdfium-render` + `pdfium-auto` crates) |
| Markdown Parser | `pulldown-cmark` |
| PDF Viewer | Custom (PNG images in scrollable div) |

---

## Performance Optimization Plan

Planned sequential performance improvements. Each fix will be implemented one at a time, with user validation between each.

| Fix | Description | Expected Impact |
|-----|-------------|---------------|
| 1 | **Background threads (`spawn_blocking`)** — Wrap `convert_md_to_pdf` and `render_pdf_pages` in `tokio::task::spawn_blocking` so Rust computation runs off the Tauri main thread | Prevents UI freeze during heavy Typst compilation or PDFium rendering; "Rendering..." spinner stays responsive |
| 2 | **Viewport page rendering** — Track which pages are visible in the preview scroll container via IntersectionObserver; only invoke PDFium for visible pages | Dramatically reduces render time for multi-page documents; initial load shows first page instantly, subsequent pages render on scroll |
| 3 | **Page caching** — Hash each page's source content + template + zoom; store rendered PNGs in a hash-keyed cache (in-memory or `AppData` temp dir); skip PDFium for unchanged pages | Near-instant re-render when only some pages changed; zoom changes are free (already handled by CSS `zoom`); big win for large documents with minor edits |
| 4 | **Persistent Typst World** — Keep a single `TypstWrapperWorld` instance across compilations instead of creating a new one per `convert_md_to_pdf` call; use Typst's built-in incremental compilation (source change tracking, cached evaluation) | Cuts Typst compilation time from ~200ms to ~20ms for small edits; fonts and packages stay loaded |
| 5 | **SVG output** — Replace the PNG pipeline entirely: render Typst to SVG instead of PDF→PNG; display SVG `<img>` or inline `<svg>` in the preview pane | Eliminates PDFium binary download (~10 MB) and per-page bitmap rasterization; SVG is vector → crisp at all zooms with zero render cost; preview becomes selectable text and searchable |

### Git History (Cleaned)

Run `git log --oneline` to verify. The unique commits on `main` are:

```
a975466 fix: escape typst delimiters { } in markdown-to-typst converter, escape table cells and link text, handle backticks in inline code
e23b22e fix: debounced zoom re-render with smooth visual feedback and old-image persistence
a7d03cf replace iframe pdf preview with rust pdfium png renderer
cedf177 fix: hide native pdf toolbar with #toolbar=0, simplify zoom wrapper with top-left origin
ccc4408 fix: use transform scale wrapper for zoom, add load timeout fallback, explicit zoom handlers with logging
973a4fc fix: open pdf in system preview, use css zoom on iframe, remove bg-white mat
824e71f fix: line number alignment, zoom controls + reveal in finder, one-directional scroll sync
c16f2fa add readme
1d10f32 add fs write permissions to tauri capabilities for export pdf
1c96190 fix: add export pdf fallback with diagnostics, robust drag handle with window listeners and iframe pointer-events guard
d678147 fix: codemirror scrollbar, scrollpastend, word wrap toggle, and gutter background
2c94813 fix: remove transition-all from split panes, store pdf bytes for export, skip compilation on empty markdown
00a10cb implement phase 5: keyboard shortcuts, toast notifications, dark mode polish, smooth transitions, and ux improvements
081eb81 fix folder-mode live preview: use editor content as single source of truth for pdf generation
8447e7b implement phase 4: wikilink resolution, merged editor view, and smart scroll sync
4f13e58 implement phase 3: vue frontend with codemirror 6 editor, live pdf preview, split view, dark mode, and file/folder open flow
0542ad9 implement phase 2: rust backend typst pipeline with md-to-pdf conversion, file commands, and 3 built-in templates
35c52ec scaffold tauri v2 + vue 3 + typescript project with tailwind, shadcn-vue, codemirror 6, and tauri plugins
```

### Files That Should NOT Be Committed

These are agent workspace files. They are present in the working directory but excluded from git:
- `agent-info.md`
- `handoff.md`
- `context/` (task_plan.md, findings.md, progress.md)

Do NOT add them to `.gitignore` — the agent exercises judgment on what to commit.

---

## Performance Optimization Progress

### Fix 1: Background Threads (`spawn_blocking`) — COMPLETED ✅
All Tauri commands converted to `async fn` with `tokio::task::spawn_blocking`. Typst compilation and PDFium rendering now run on the blocking thread pool. UI stays fully responsive during heavy computation. User validated.

### Fix 2: Viewport Page Rendering — COMPLETED ✅
Added `render_pdf_page_range` and `get_pdf_page_count` Rust commands. Rewrote `usePdfRenderer.ts` with a module-scoped page cache and lazy rendering. `PreviewPane.vue` uses `IntersectionObserver` with 200px buffer to detect visible pages and shows gray placeholders for uncached pages. Only pages near the viewport are rendered; scrolling loads more on-demand. Cache persists across scrolls for instant back-navigation. Initial bug (all-gray placeholders) was caused by a non-reactive `Map` in a computed property; fixed by returning the reactive `pages` ref directly. User validated.

### Fix 3: Page Caching by Content Hash — IN PROGRESS
Implemented persistent cross-compile cache keyed by `(pdfHash, pageNum, zoom)` with stale-while-revalidate UX. Old pages stay visible during recompile; a sticky "Recompiling..." badge indicates stale state. Cache persists across document switches via module-scoped Map. SHA-256 hash of PDF bytes determines cache hits. Awaiting user validation.

### Fix 4: Persistent Typst World — READY TO START
Keep the Typst compiler alive between invocations, update only changed source. Text-only edits drop from ~500ms to ~50-150ms.

### Fix 5: SVG Output — READY TO START
Replace PNG pipeline entirely with SVG output from Typst. Near-instant updates, selectable text, smallest bundle.
