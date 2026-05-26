use std::path::PathBuf;

mod md_to_typst;
mod pdfium_renderer;
mod typst_world;
mod wikilinks;

use typst_world::TypstWrapperWorld;
use typst_pdf::PdfOptions;

const TEMPLATE_DIR: &str = "templates";
const MARKER: &str = "// MARKTASTIC_BODY_CONTENT";

/// Compile markdown to PDF using a built-in template.
#[tauri::command]
async fn convert_md_to_pdf(markdown: String, template_name: String) -> Result<Vec<u8>, String> {
    tokio::task::spawn_blocking(move || {
        compile_markdown_to_pdf(&markdown, &template_name)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Compile a folder of linked markdown files to PDF.
/// Resolves wikilinks, merges reachable files, and compiles to PDF.
#[tauri::command]
async fn compile_folder_to_pdf(folder_path: String, template_name: String) -> Result<Vec<u8>, String> {
    tokio::task::spawn_blocking(move || {
        let (merged_markdown, _boundaries) = wikilinks::build_merged_document(&folder_path)?;
        compile_markdown_to_pdf(&merged_markdown, &template_name)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Resolve wikilinks in a folder and return the ordered list of reachable file names.
#[tauri::command]
async fn resolve_wikilinks(folder_path: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        wikilinks::resolve_wikilinks(&folder_path)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Shared PDF compilation logic.
fn compile_markdown_to_pdf(markdown: &str, template_name: &str) -> Result<Vec<u8>, String> {
    // Convert markdown to Typst markup
    let typst_body = md_to_typst::convert_md_to_typst(markdown);

    // Resolve template path
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();

    // Try multiple locations for the template (dev vs release builds)
    let template_path_candidates = [
        exe_dir.join(TEMPLATE_DIR).join(format!("{}.typ", template_name)),
        exe_dir.join("../..").join(TEMPLATE_DIR).join(format!("{}.typ", template_name)),
        PathBuf::from(format!("src-tauri/{}/{}", TEMPLATE_DIR, template_name)),
        PathBuf::from(format!("{}/{}", TEMPLATE_DIR, template_name)),
    ];

    let template_content = {
        let mut found = None;
        for path in &template_path_candidates {
            let path_with_ext = if path.extension().is_none() {
                path.with_extension("typ")
            } else {
                path.clone()
            };
            if path_with_ext.exists() {
                found = Some(
                    std::fs::read_to_string(&path_with_ext)
                        .map_err(|e| format!("Failed to read template file: {}", e))?
                );
                break;
            }
        }
        found.ok_or_else(|| format!(
            "Template '{}' not found. Searched: {:?}",
            template_name,
            template_path_candidates
        ))?
    };

    // Inject body into template
    let full_source = if template_content.contains(MARKER) {
        template_content.replace(MARKER, &typst_body)
    } else {
        format!("{}\n{}", template_content, typst_body)
    };

    // Compile with Typst
    let world = TypstWrapperWorld::new(
        exe_dir.to_string_lossy().to_string(),
        full_source,
    );

    let document = typst::compile(&world)
        .output
        .map_err(|diags| {
            let messages: Vec<String> = diags.iter().map(|d| format!("{:?}", d)).collect();
            format!("Typst compilation failed: {}", messages.join("; "))
        })?;

    // Export to PDF
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| format!("PDF export failed: {:?}", e))?;

    Ok(pdf)
}

#[tauri::command]
async fn open_file_path(file_path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn open_folder(folder_path: String) -> Result<Vec<(String, String)>, String> {
    tokio::task::spawn_blocking(move || {
        let mut results = Vec::new();
        let entries = std::fs::read_dir(&folder_path)
            .map_err(|e| format!("Failed to read folder '{}': {}", folder_path, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read '{}': {}", path.display(), e))?;
                results.push((file_name, content));
            }
        }

        Ok(results)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn get_templates() -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let exe_dir = std::env::current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?
            .parent()
            .ok_or("Failed to get executable directory")?
            .to_path_buf();

        let template_dir_candidates = [
            exe_dir.join(TEMPLATE_DIR),
            exe_dir.join("../..").join(TEMPLATE_DIR),
            PathBuf::from(format!("src-tauri/{}", TEMPLATE_DIR)),
            PathBuf::from(TEMPLATE_DIR),
        ];

        let mut templates = Vec::new();
        for dir in &template_dir_candidates {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "typ").unwrap_or(false) {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            templates.push(stem.to_string());
                        }
                    }
                }
                if !templates.is_empty() {
                    break;
                }
            }
        }

        if templates.is_empty() {
            // Fallback: return known templates
            templates = vec![
                "basic-report".to_string(),
                "university-assignment".to_string(),
                "thesis-chapter".to_string(),
            ];
        }

        Ok(templates)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Render PDF pages to PNG images using PDFium.
#[tauri::command]
async fn render_pdf_pages(
    pdf_bytes: Vec<u8>,
    zoom: f64,
    device_pixel_ratio: f64,
) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        pdfium_renderer::render_pdf_pages(&pdf_bytes, zoom, device_pixel_ratio)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Get the total number of pages in a PDF.
#[tauri::command]
async fn get_pdf_page_count(pdf_bytes: Vec<u8>) -> Result<u16, String> {
    tokio::task::spawn_blocking(move || {
        pdfium_renderer::get_pdf_page_count(&pdf_bytes)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Render only specific pages of a PDF to PNG images.
#[tauri::command]
async fn render_pdf_page_range(
    pdf_bytes: Vec<u8>,
    page_numbers: Vec<usize>,
    zoom: f64,
    device_pixel_ratio: f64,
) -> Result<Vec<(usize, String)>, String> {
    tokio::task::spawn_blocking(move || {
        pdfium_renderer::render_pdf_page_range(&pdf_bytes, page_numbers, zoom, device_pixel_ratio)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            convert_md_to_pdf,
            compile_folder_to_pdf,
            resolve_wikilinks,
            open_file_path,
            open_folder,
            get_templates,
            render_pdf_pages,
            get_pdf_page_count,
            render_pdf_page_range,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
