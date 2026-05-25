use std::path::PathBuf;

mod md_to_typst;
mod typst_world;

use typst_world::TypstWrapperWorld;
use typst_pdf::PdfOptions;

const TEMPLATE_DIR: &str = "templates";
const MARKER: &str = "// MARKTASTIC_BODY_CONTENT";

#[tauri::command]
fn convert_md_to_pdf(markdown: String, template_name: String) -> Result<Vec<u8>, String> {
    // Convert markdown to Typst markup
    let typst_body = md_to_typst::convert_md_to_typst(&markdown);

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
fn open_file_path(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))
}

#[tauri::command]
fn open_folder(folder_path: String) -> Result<Vec<(String, String)>, String> {
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
}

#[tauri::command]
fn get_templates() -> Result<Vec<String>, String> {
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
            open_file_path,
            open_folder,
            get_templates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
