# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Live PDF Preview** — Real-time preview of Markdown rendered as PDF with 500ms debounce
- **Wikilink Support** — Multi-file document support using `[[filename]]` syntax with BFS dependency resolution
- **Three Typst Templates**:
  - Basic Report
  - University Assignment
  - Thesis Chapter
- **CodeMirror 6 Editor** — Syntax highlighting for Markdown, line numbers, word wrap toggle
- **Split View Mode** — Resizable horizontal split between editor and preview panes
- **Dark Mode** — Three themes: Light, Dark, and System (follows OS preference)
- **Keyboard Shortcuts**:
  - `Ctrl/Cmd + O` — Open file
  - `Ctrl/Cmd + Shift + O` — Open folder
  - `Ctrl/Cmd + S` — Export PDF
  - `Ctrl/Cmd + E/P/B` — Editor-only / Preview-only / Both panes
  - `Ctrl/Cmd + T` — Cycle theme
  - `Ctrl/Cmd + Shift + W` — Toggle word wrap
- **Toast Notifications** — Non-blocking notifications for compile status, errors, and file operations
- **Window Title** — Dynamic window title showing current file or folder name
- **App Icon** — Custom icon with transparent background, proper padding for all platforms
- **Three-Pane Layout** — Toolbar header with template selector, resizable split view, floating zoom/pane controls

### Changed

- **SVG-based Rendering** — Replaced PDFium PNG pipeline with direct Typst SVG output for near-instant preview
- **Persistent Typst World** — Font search cached across compiles for faster compilation
- **Page Caching** — Content-hash-based caching with stale-while-revalidate strategy
- **Background Compilation** — Rust commands run on background threads via `spawn_blocking`
- **Export PDF** — Direct on-demand PDF compilation (no cached PDF bytes)

### Fixed

- **Editor Freeze on Type** — Multiple fixes:
  - Replaced `v-html` SVG injection with `<img>` data URLs (browser parses SVG off main thread)
  - Added `requestIdleCallback` yield before DOM injection
  - Content dedupe to skip redundant recompiles
- **Broken Wikilinks** — Warning toast shows count of broken links when opening folders
- **Split Pane Resize** — Fixed drag continuation after mouse release (uses `window` listeners)
- **Typst Delimiter Escaping** — `{`, `}`, and backticks properly escaped in Markdown-to-Typst converter
- **Line Number Alignment** — Fixed misalignment with padded content
- **Zoom Controls** — Instant zoom response via CSS `zoom` property

### Removed

- **PDFium PNG Pipeline** — Replaced with direct SVG rendering
- **iframe PDF Viewer** — No longer needed with SVG output
- **Open in System Preview** — Removed unused button and related composables
- **Unused Composables** — `usePdf`, `usePdfRenderer` (285 lines of dead code)

## [0.1.0] — 2026-05-27

### Added

- Initial release with Tauri v2 + Vue 3 + TypeScript scaffold
- CodeMirror 6 editor integration
- Three built-in Typst templates
- Tailwind CSS and shadcn-vue styling
- Markdown to Typst to PDF pipeline in Rust
- Resizable split view with zoom controls

[unreleased]: https://github.com/abhinavborah/marktastic/compare/v0.1.0...main
[0.1.0]: https://github.com/abhinavborah/marktastic/releases/tag/v0.1.0
