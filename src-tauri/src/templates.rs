//! Template management for Marktastic.
//!
//! Handles two-tier template system:
//! - User templates: `~/.marktastic/templates/` (created on first use)
//! - Bundled templates: `src-tauri/templates/` (read-only, bundled in app)

use std::path::PathBuf;

/// Returns the user templates directory (`~/.marktastic/templates/`) and creates it if missing.
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the user templates directory
/// * `Err(String)` - Error message if home directory not found or directory creation failed
pub fn get_user_templates_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    
    let templates_dir = home.join(".marktastic").join("templates");
    
    if !templates_dir.exists() {
        std::fs::create_dir_all(&templates_dir)
            .map_err(|e| format!("Failed to create templates directory: {}", e))?;
    }
    
    Ok(templates_dir)
}

/// Returns the bundled templates directory path.
///
/// In production: `app.app/Contents/Resources/templates/`
/// In development: `src-tauri/templates/`
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the bundled templates directory
/// * `Err(String)` - Error message if directory not found
pub fn get_bundled_templates_dir() -> Result<PathBuf, String> {
    // For bundled templates, we need to look relative to the executable
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get executable directory")?;
    
    // Try multiple possible locations (production and development)
    let bundled_paths = [
        // Production macOS: app.app/Contents/Resources/templates/
        exe_dir.join("Resources").join("templates"),
        // macOS dev: target/debug/marktastic/templates/
        exe_dir.join("templates"),
        // Fallback: relative to current working directory (dev)
        PathBuf::from("src-tauri/templates"),
        PathBuf::from("templates"),
    ];
    
    for path in &bundled_paths {
        if path.exists() && path.is_dir() {
            return Ok(path.clone());
        }
    }
    
    Err("Bundled templates directory not found".to_string())
}

/// Get all template names from both user and bundled directories.
///
/// # Returns
/// * `Ok(Vec<String>)` - List of template names (without .typ extension)
/// * `Err(String)` - Error message
pub fn get_all_template_names() -> Result<Vec<String>, String> {
    let mut templates = Vec::new();
    
    // Add bundled templates first
    if let Ok(bundled_dir) = get_bundled_templates_dir() {
        if let Ok(entries) = std::fs::read_dir(&bundled_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".typ") {
                        let template_name = name.trim_end_matches(".typ").to_string();
                        templates.push(template_name);
                    }
                }
            }
        }
    }
    
    // Add user templates (user templates can override bundled ones)
    if let Ok(user_dir) = get_user_templates_dir() {
        if user_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&user_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".typ") {
                            // Remove if already added from bundled (override)
                            let template_name = name.trim_end_matches(".typ").to_string();
                            templates.retain(|t| t != &template_name);
                            templates.push(template_name);
                        }
                    }
                }
            }
        }
    }
    
    // Sort and return
    templates.sort();
    Ok(templates)
}

/// Check if a template is a built-in (bundled) template.
///
/// # Arguments
/// * `name` - Template name (without .typ extension)
///
/// # Returns
/// * `bool` - True if the template exists in the bundled directory
pub fn is_bundled_template(name: &str) -> bool {
    if let Ok(bundled_dir) = get_bundled_templates_dir() {
        let template_path = bundled_dir.join(format!("{}.typ", name));
        return template_path.exists();
    }
    false
}

/// Find a template by name using two-tier lookup.
///
/// Lookup order:
/// 1. User templates (`~/.marktastic/templates/{name}.typ`) - user templates take precedence
/// 2. Bundled templates (`Resources/templates/{name}.typ` or `src-tauri/templates/{name}.typ`)
///
/// # Arguments
/// * `name` - Template name (without .typ extension)
///
/// # Returns
/// * `Ok(String)` - The template content
/// * `Err(String)` - Error if template not found or read failed
pub fn find_template(name: &str) -> Result<String, String> {
    // First: Check user templates (user overrides bundled)
    if let Ok(user_dir) = get_user_templates_dir() {
        let user_path = user_dir.join(format!("{}.typ", name));
        if user_path.exists() {
            return std::fs::read_to_string(&user_path)
                .map_err(|e| format!("Failed to read user template: {}", e));
        }
    }
    
    // Second: Check bundled templates
    let bundled_dir = get_bundled_templates_dir()?;
    let bundled_path = bundled_dir.join(format!("{}.typ", name));
    if bundled_path.exists() {
        return std::fs::read_to_string(&bundled_path)
            .map_err(|e| format!("Failed to read bundled template: {}", e));
    }
    
    Err(format!("Template '{}' not found", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_directory_creation() {
        let result = get_user_templates_dir();
        assert!(result.is_ok(), "Should create user templates directory");
        
        let dir = result.unwrap();
        assert!(dir.exists(), "Directory should exist after creation");
        assert!(dir.is_dir(), "Should be a directory");
    }

    #[test]
    fn test_bundled_directory() {
        let result = get_bundled_templates_dir();
        assert!(result.is_ok(), "Bundled templates directory should exist");
    }

    #[test]
    fn test_template_names() {
        let names = get_all_template_names().unwrap();
        assert!(!names.is_empty(), "Should have at least bundled templates");
        assert!(names.contains(&"basic-report".to_string()), "Should have basic-report template");
    }
}
