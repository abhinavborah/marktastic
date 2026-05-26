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
- Live preview: **SVG** (Typst-compiled) rendered as raw HTML, not PNG

## Context Files to Read
Before starting work, read:
- `context/task_plan.md` — full task plan and requirements
- `context/findings.md` — research findings and technical decisions
- `context/progress.md` — session log and current status

## Skills to Load
- `planning-with-files` — for structured planning
- `context7` — for Tauri v2 and Typst crate documentation lookups

## Next Actions (Phase 6)
**Current state:** P0 Fixes 1 & 2 committed (48af1aa). P0 Fix 3 (debounce) remaining.
**Next agent action:** Implement P0 Fix 3 (debounce verification + dedupe in useSvgRenderer).

1. Cross-platform build configuration (`tauri.conf.json` bundle settings for macOS, Windows, Linux)
2. GitHub Actions CI workflow for automated builds on push/tag
3. GitHub Releases setup with signed binaries
4. App code signing configuration (macOS notarization, Windows code signing if available)
5. Update `README.md` with installation and usage instructions
6. Tag v0.1.0 release

---

## P0 Performance Fixes (Live Preview Freezing)

Implemented to fix app freezing when typing in the editor.

### P0 Fix 1 & 2: ✅ Both committed in 48af1aa
**Problem:** Two causes of main-thread blocking causing editor freeze:
1. Vue `v-html` parsed SVG XML into DOM nodes on main JS thread
2. Synchronous `pages.value = result` caused Vue to update DOM immediately after Rust response

**Solution (committed):**
- `PreviewPane.vue`: Replaced `v-html` with `<img>` tags using SVG data URLs — browser parses SVG off the main JS thread
- `useSvgRenderer.ts`: Added `requestIdleCallback` yield before `pages.value = result` — gives CodeMirror time to process pending keystrokes before DOM update

**Files changed:** `src/components/PreviewPane.vue`, `src/composables/useSvgRenderer.ts`
**Commit:** `48af1aa` — "replace v-html svg injection with img data urls to prevent main thread blocking"

---

## Remaining P0 Fixes (Not Yet Implemented)

### P0 Fix 3: Debounce verification + dedupe in useSvgRenderer
**Problem:** Vue `watch` doesn't have a built-in debounce option. The `{ debounce: 500 }` option in `watch()` is invalid Vue API — compilation may not be properly debounced.
**Solution:** Implement manual debounce with `setTimeout`/`clearTimeout`. Also add dedupe to prevent recompile of identical documents.
**Files likely to change:** `src/composables/useSvgRenderer.ts`, `src/components/EditorPane.vue`
**Test:** Type in editor → preview updates after ~300ms of inactivity, not instantly.

---

## P1 Speed Fixes (Planned)

Implement after all P0 fixes are validated:

### P1 Fix 1: Lower debounce to 300ms with 500ms maxWait
**Problem:** 500ms debounce feels sluggish for a live preview.
**Solution:** Change debounce to 300ms with a maxWait of 500ms (fires at least every 500ms during continuous typing).
**Files likely to change:** `src/composables/useSvgRenderer.ts`

### P1 Fix 2: Request versioning (cancel stale results)
**Problem:** If user types while a compile is running, the old result can overwrite the new result when it finally arrives.
**Solution:** Add a version counter. Each compile increments the version. If returned version < current version, discard result.
**Files likely to change:** `src/composables/useSvgRenderer.ts`, `src-tauri/src/lib.rs`

---

## Major Changes Since Last Handoff

### Architecture Change: iframe → Rust PDFium Renderer → SVG
The original architecture displayed PDFs via `<iframe src="blob:...">`. This was replaced with a Rust-based PDFium renderer, then replaced again with direct Typst SVG output:
- **Old flow:** Markdown → Typst → PDF bytes → PDFium → PNG → Vue `<img>` tags
- **New flow:** Markdown → Typst → SVG strings → Vue `<div v-html>`
- `convert_md_to_svg` Rust command compiles directly to SVG using `typst-svg` crate
- `useSvgRenderer.ts` composable watches editor content + template, debounces 500ms, calls `convert_md_to_svg`
- `PreviewPane.vue` renders raw SVG markup via `v-html` with CSS zoom
- PDF export still uses `convert_md_to_pdf` + `typst-pdf`

### Performance Optimizations (All 5 Fixes)

| Fix | Description | Status |
|-----|-------------|--------|
| 1 | Background threads (`spawn_blocking`) | ✅ COMPLETED |
| 2 | Viewport page rendering (lazy loading) | ✅ COMPLETED |
| 3 | Page caching by content hash (stale-while-revalidate) | ✅ COMPLETED |
| 4 | Persistent Typst World (eliminates per-compile font search) | ✅ COMPLETED |
| 5 | SVG output (replaces PDFium PNG pipeline) | ✅ COMPLETED |

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
   - Fix: Multiple iterations — first CSS scale debounce, then fixed 2.0× pre-render, then final SVG output which eliminates the issue entirely

8. **Typst "unclosed delimiter" errors on large markdown files**
   - Root cause: `escape_text` didn't escape `{` and `}`, which are Typst content block delimiters
   - Fix: Added `{` and `}` escaping to both `escape_text` and `escape_string`; also fixed table cells and link text to use `escape_text` consistently; added `typst_raw_string()` for inline code containing backticks

9. **Loading overlay stuck on preview** *(iframe-era issue, now moot)*
   - Root cause: iframe `@load` event sometimes failed to fire for blob URLs
   - Fix (at the time): Added 3-second timeout fallback (`loadTimedOut`) that hid the overlay if load never completed
   - **Current state:** With SVG output, loading state is driven by the `rendering` boolean from `useSvgRenderer`; no timeout mechanism is needed

10. **White overlay at bottom of preview in dark mode** *(iframe-era issue, now moot)*
    - Root cause: iframe had a `bg-white` class; when the PDF page was shorter than the viewport, white showed below
    - Fix (at the time): Removed `bg-white` from the iframe
    - **Current state:** SVG preview has transparent background; page content has its own background

11. **Bottom-center native PDF viewer toolbar visible** *(iframe-era issue)*
    - Root cause: `#toolbar=0` URL fragment is Adobe-specific, ignored by Apple's PDFKit in WKWebView
    - Fix: Eliminated the iframe entirely — replaced with the PDFium PNG renderer, then replaced again with SVG output

12. **"Reveal in Finder" → "Open in Preview"**
    - Original implementation used `@tauri-apps/plugin-shell` to open a folder in Finder
    - Changed to "Open in Preview" — writes the current PDF bytes to a temp file (`marktastic-preview.pdf` in the system temp directory) and opens it with the system default PDF app via `openInShell()`

13. **CSS `zoom` property doesn't work on iframe PDF**
    - Root cause: CSS `zoom` is non-standard and ignored by iframe PDF content
    - Fix: With SVG output, zoom is proper — CSS `transform: scale()` on the SVG container, which the browser handles natively

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
│       │   └── Scrollable SVG list (raw SVG via v-html)
│       │
│       └── floating controls (top-right)
│           ├── Editor-only / Split / Preview-only buttons
│           ├── Word wrap toggle
│           ├── Zoom out / zoom percent / zoom in buttons
│           └── Open in Preview button
│
└── ToastContainer (inside PreviewPane, top-center)
    └── Stacked pill-badge toast notifications
```

### Data Flow

```
User opens file/folder
    ↓
Tauri dialog → Rust command (open_file_path / open_folder)
    ↓
editorContent (Vue ref)
    ↓
useSvgRenderer: watches editorContent + selectedTemplate → debounce 500ms → invoke('convert_md_to_svg')
    ↓
Rust: md_to_typst → persistent TypstWorld → typst::compile → typst_svg::svg per page → Vec<String>
    ↓
pages (Vue ref: string[] of raw SVG markup)
    ↓
PreviewPane: v-for page in pages → <div v-html="page"> with CSS zoom
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

1. **First TypstWorld compile** — Font search happens once at startup (~100-300ms). Subsequent compiles skip it.
2. **SVG rendering** — Some complex Typst features (like embedded images with filters) may not render correctly in all browsers. Standard text/layout/table features work well.
3. **PDF export** — Still uses `convert_md_to_pdf` which compiles separately from SVG preview. Two compilations on edit (one for preview SVG, one for export PDF). Could be optimized to a single compile that outputs both formats.
4. **Print support** — Not implemented. Would need a separate print stylesheet or PDF export + print.
5. **Auto-save** — Not implemented.
6. **File sidebar for folder mode** — Not implemented (would require tracking section boundaries in the editor for clickable navigation).
7. **Custom user-defined Typst templates** — Not yet supported. Templates are hardcoded in `src-tauri/templates/`.

### Tech Stack (Current)

| Layer | Technology |
|-------|------------|
| Backend | Rust + Tauri v2 |
| Frontend | Vue 3 + Vite + TypeScript |
| UI | shadcn-vue + Tailwind CSS |
| Editor | CodeMirror 6 |
| PDF Generation | Typst (`typst`, `typst-pdf`, `typst-kit` crates) |
| SVG Preview | Typst SVG (`typst-svg` crate) |
| Markdown Parser | `pulldown-cmark` |
| PDF Viewer | Native SVG in browser (via `v-html`) |

### Git History (Cleaned)

Run `git log --oneline` to verify. Key commits include scaffolding, Typst pipeline, Vue frontend, wikilinks, dark mode polish, PDFium renderer, debounced zoom, persistent world, and SVG output.

### Files That Should NOT Be Committed

These are agent workspace files. They are present in the working directory but excluded from git:
- `agent-info.md`
- `handoff.md`
- `context/` (task_plan.md, findings.md, progress.md)

Do NOT add them to `.gitignore` — the agent exercises judgment on what to commit.
