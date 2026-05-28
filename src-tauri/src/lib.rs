use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod md_to_typst;
mod pdfium_renderer;
mod typst_world;
mod wikilinks;
mod templates;

use templates::{find_template, get_user_templates_dir, is_bundled_template};
use typst_world::TypstWrapperWorld;
use typst_pdf::PdfOptions;

const MARKER: &str = "// MARKTASTIC_BODY_CONTENT";

/// Persistent Typst compilation state.
struct AppState {
    world: Arc<Mutex<TypstWrapperWorld>>,
}

/// Build the full Typst source by resolving template and injecting body.
fn build_full_source(typst_body: &str, template_name: &str) -> Result<String, String> {
    // Use two-tier template lookup (user templates override bundled)
    let template_content = find_template(template_name)?;

    let full_source = if template_content.contains(MARKER) {
        template_content.replace(MARKER, typst_body)
    } else {
        format!("{}\n{}", template_content, typst_body)
    };

    Ok(full_source)
}

/// Compile using the persistent world. The world is locked for the duration.
fn compile_with_world(world: &mut TypstWrapperWorld, source: &str) -> Result<Vec<u8>, String> {
    world.update_source(source.to_string());

    let document = typst::compile(world)
        .output
        .map_err(|diags| {
            let messages: Vec<String> = diags.iter().map(|d| format!("{:?}", d)).collect();
            format!("Typst compilation failed: {}", messages.join("; "))
        })?;

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| format!("PDF export failed: {:?}", e))?;

    Ok(pdf)
}

/// Compile markdown to PDF using a built-in template.
#[tauri::command]
async fn convert_md_to_pdf(
    state: tauri::State<'_, AppState>,
    markdown: String,
    template_name: String,
) -> Result<Vec<u8>, String> {
    let world_arc = state.world.clone();
    tokio::task::spawn_blocking(move || {
        let typst_body = md_to_typst::convert_md_to_typst(&markdown);
        let full_source = build_full_source(&typst_body, &template_name)?;
        let mut world = world_arc.lock().map_err(|e| format!("World lock error: {}", e))?;
        compile_with_world(&mut world, &full_source)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Compile a folder of linked markdown files to PDF.
/// Resolves wikilinks, merges reachable files, and compiles to PDF.
#[tauri::command]
async fn compile_folder_to_pdf(
    state: tauri::State<'_, AppState>,
    folder_path: String,
    template_name: String,
) -> Result<Vec<u8>, String> {
    let world_arc = state.world.clone();
    tokio::task::spawn_blocking(move || {
        let (merged_markdown, _boundaries) = wikilinks::build_merged_document(&folder_path)?;
        let typst_body = md_to_typst::convert_md_to_typst(&merged_markdown);
        let full_source = build_full_source(&typst_body, &template_name)?;
        let mut world = world_arc.lock().map_err(|e| format!("World lock error: {}", e))?;
        compile_with_world(&mut world, &full_source)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

fn compile_to_svg(world: &mut TypstWrapperWorld, source: &str) -> Result<Vec<String>, String> {
    world.update_source(source.to_string());

    let document: typst::layout::PagedDocument = typst::compile(world)
        .output
        .map_err(|diags| {
            let messages: Vec<String> = diags.iter().map(|d| format!("{:?}", d)).collect();
            format!("Typst compilation failed: {}", messages.join("; "))
        })?;

    let mut pages = Vec::new();
    for page in document.pages.iter() {
        let svg = typst_svg::svg(page);
        pages.push(svg);
    }

    Ok(pages)
}

/// Compile markdown to SVG pages using a built-in template.
#[tauri::command]
async fn convert_md_to_svg(
    state: tauri::State<'_, AppState>,
    markdown: String,
    template_name: String,
) -> Result<Vec<String>, String> {
    let world_arc = state.world.clone();
    tokio::task::spawn_blocking(move || {
        let typst_body = md_to_typst::convert_md_to_typst(&markdown);
        let full_source = build_full_source(&typst_body, &template_name)?;
        let mut world = world_arc.lock().map_err(|e| format!("World lock error: {}", e))?;
        compile_to_svg(&mut world, &full_source)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
#[tauri::command]
async fn resolve_wikilinks(folder_path: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        wikilinks::resolve_wikilinks(&folder_path)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
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
        // Use merged template list (user + bundled) from templates module
        templates::get_all_template_names()
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

/// Get the content of a template for editing.
#[tauri::command]
fn get_template_content(template_name: String) -> Result<String, String> {
    find_template(&template_name)
}

/// Save or update a user template (fails for built-in templates).
#[tauri::command]
fn save_user_template(template_name: String, content: String) -> Result<(), String> {
    if is_bundled_template(&template_name) {
        return Err("Cannot modify built-in templates".to_string());
    }
    let user_dir = get_user_templates_dir()?;
    let template_path = user_dir.join(format!("{}.typ", template_name));
    std::fs::write(&template_path, &content)
        .map_err(|e| format!("Failed to save template: {}", e))?;
    Ok(())
}

/// Delete a user template (fails for built-in templates).
#[tauri::command]
fn delete_user_template(template_name: String) -> Result<(), String> {
    if is_bundled_template(&template_name) {
        return Err("Cannot delete built-in templates".to_string());
    }
    let user_dir = get_user_templates_dir()?;
    let template_path = user_dir.join(format!("{}.typ", template_name));
    if !template_path.exists() {
        return Err(format!("Template '{}' not found", template_name));
    }
    std::fs::remove_file(&template_path)
        .map_err(|e| format!("Failed to delete template: {}", e))?;
    Ok(())
}

/// Export a template to a destination file.
#[tauri::command]
fn export_template(template_name: String, destination: String) -> Result<(), String> {
    let content = find_template(&template_name)?;
    std::fs::write(&destination, &content)
        .map_err(|e| format!("Failed to export template: {}", e))?;
    Ok(())
}

/// Import a template from a .typ file.
#[tauri::command]
fn import_template(source_path: String) -> Result<String, String> {
    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err(format!("File not found: {}", source_path));
    }
    let content = std::fs::read_to_string(&source)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    let file_stem = source.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?
        .to_string();
    let user_dir = get_user_templates_dir()?;
    let dest = user_dir.join(format!("{}.typ", file_stem));
    std::fs::write(&dest, &content)
        .map_err(|e| format!("Failed to save imported template: {}", e))?;
    Ok(file_stem)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let exe_dir = std::env::current_exe()
        .expect("failed to get executable path")
        .parent()
        .expect("failed to get executable directory")
        .to_path_buf();

    let world = TypstWrapperWorld::new(
        exe_dir.to_string_lossy().to_string(),
        String::new(),
    );

    tauri::Builder::default()
        .manage(AppState {
            world: Arc::new(Mutex::new(world)),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            convert_md_to_pdf,
            compile_folder_to_pdf,
            convert_md_to_svg,
            resolve_wikilinks,
            open_file_path,
            open_folder,
            get_templates,
            get_template_content,
            save_user_template,
            delete_user_template,
            export_template,
            import_template,
            render_pdf_pages,
            get_pdf_page_count,
            render_pdf_page_range,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
