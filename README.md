# Marktastic

> Convert Markdown to beautiful PDFs

![marktastic-app](./demo/marktastic-app.png)

[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8D8?logo=tauri)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3-4FC08D?logo=vuedotjs)](https://vuejs.org)
[![Typst](https://img.shields.io/badge/Typst-0.14-239DAD?logo=typst)](https://typst.app)
[![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)](https://www.rust-lang.org)

## Quickstart

![installation-guide](./demo/marktastic-installation-guide.png)

## Description

**Marktastic** is a cross-platform desktop application that transforms Markdown — including GitHub Flavored Markdown and wikilinks — into professionally typeset PDFs using [Typst](https://typst.app). Unlike tools that preview rendered HTML, Marktastic shows a **live PDF preview** so you see exactly what will be printed.

Key differentiators:

- **Live PDF preview** — not HTML, actual PDF output rendered in real time
- **Wikilink support** — write multi-file projects with `[[filename]]` links that merge into a single PDF
- **Built-in Typst templates** — professional layouts out of the box

Available for **macOS** (**Windows** and **Linux** coming soon).

---

## Features

| Feature                         | Description                                                                                                    |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| **Live PDF Preview**            | See your PDF update in real time as you type (500 ms debounce)                                                 |
| **CodeMirror 6 Editor**         | Full-featured Markdown editor with syntax highlighting, line numbers, and word wrap toggle                     |
| **Split View**                  | Resizable side-by-side editor and preview with pane mode toggles                                               |
| **Wikilink Support**            | Open folders with `[[filename]]` wikilinks; all linked Markdown files merge into one PDF                       |
| **Built-in Templates**          | 3 professional Typst templates: Basic Report, University Assignment, Thesis Chapter                            |
| **Dark Mode**                   | 3 modes: Light, Dark, System (follows OS preference)                                                           |
| **Keyboard Shortcuts**          | Full shortcut coverage for file operations, pane toggles, theme cycling, and more                              |
| **Multi-file Projects**         | Open a folder with an `index.md` or `main.md` entry point; wikilinks automatically resolve sibling `.md` files |
| **Toast Notifications**         | Visual feedback for file operations, compilation status, and export success or failure                         |
| **Smart Scroll Sync**           | Editor and preview scroll positions stay roughly synchronized                                                  |
| **Customizable User Templates** | User can manage and import their custom .typ templates. Templates are stored in `~/.marktastic/templates/`     |

---

## Tech Stack

| Layer               | Technology                                                               |
| ------------------- | ------------------------------------------------------------------------ |
| **Backend**         | Rust + Tauri v2                                                          |
| **Frontend**        | Vue 3 + Vite + TypeScript                                                |
| **UI**              | shadcn-vue + Tailwind CSS                                                |
| **Editor**          | CodeMirror 6                                                             |
| **PDF Engine**      | Typst (in-process via `typst`, `typst-pdf`, and `typst-kit` Rust crates) |
| **Markdown Parser** | `pulldown-cmark`                                                         |

---

## Installation

### Download Pre-built Binaries

Check the [releases page](https://github.com/abhinavborah/marktastic/releases) for pre-built binaries.

### Build from Source

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) + Cargo (latest stable)
- [Node.js](https://nodejs.org) + npm (v18+)
- [Typst CLI](https://github.com/typst/typst) (optional, for template development)

#### Steps

```bash
# Clone the repository
git clone https://github.com/abhinavborah/marktastic
cd marktastic

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build production app
npm run tauri build
```

---

## Development

```bash
# Start development server with hot reload
npm run tauri dev

# Build frontend only
npm run build

# Check Rust compilation
cd src-tauri && cargo check

# Run production build (creates .app, .dmg, .msi, .deb, etc.)
npm run tauri build
```

---

## Keyboard Shortcuts

| Shortcut               | Action                              |
| ---------------------- | ----------------------------------- |
| `Ctrl/Cmd + O`         | Open single Markdown file           |
| `Ctrl/Cmd + Shift + O` | Open folder with Markdown files     |
| `Ctrl/Cmd + S`         | Export to PDF                       |
| `Ctrl/Cmd + E`         | Editor-only pane                    |
| `Ctrl/Cmd + P`         | Preview-only pane                   |
| `Ctrl/Cmd + B`         | Both panes (split view)             |
| `Ctrl/Cmd + T`         | Cycle theme (light → dark → system) |
| `Ctrl/Cmd + Shift + W` | Toggle word wrap                    |

---

## Multi-File Projects with Wikilinks

Marktastic supports multi-file Markdown projects via wikilinks.

1. Create a folder with your Markdown files.
2. Name your entry point `index.md` or `main.md`.
3. Link other files using `[[filename]]` syntax (the `.md` extension is optional):

example codeblock:

```markdown
# My Report

## Introduction

See [[introduction]] for background.

## Methodology

Details in [[methodology]].

## Results

Check [[results]] and [[discussion]].
```

4. Open the folder in Marktastic — all linked files merge into a single PDF.
5. Unlinked files are excluded from the output.
6. Broken links (missing files) render as _Missing link: filename_ in the PDF.

---

## Roadmap

Planned enhancements:

- [x] Custom user-defined Typst templates
- [ ] File sidebar for folder mode navigation
- [ ] Find and replace in the editor
- [ ] Auto-save
- [ ] Plugin system for custom Markdown extensions
- [ ] Robust cross-platform support
