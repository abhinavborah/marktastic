use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

lazy_static::lazy_static! {
    static ref WIKILINK_RE: Regex = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
}

/// Resolve wikilinks in a folder starting from an entry point.
/// Returns the ordered list of reachable file names (including the entry point).
pub fn resolve_wikilinks(folder_path: &str) -> Result<Vec<String>, String> {
    let folder = Path::new(folder_path);
    if !folder.is_dir() {
        return Err(format!("Not a directory: {}", folder_path));
    }

    let entry_point = find_entry_point(folder)?;
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    let mut queue = vec![entry_point.clone()];

    while let Some(file_name) = queue.pop() {
        if visited.contains(&file_name) {
            continue;
        }
        visited.insert(file_name.clone());
        order.push(file_name.clone());

        let file_path = folder.join(&file_name);
        let content = match std::fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!(
                    "Failed to read '{}': {}",
                    file_path.display(),
                    e
                ));
            }
        };

        let links = extract_wikilinks(&content);
        for link in links {
            let linked_file = normalize_link(&link);
            let linked_path = folder.join(&linked_file);
            if linked_path.exists() && !visited.contains(&linked_file) {
                queue.push(linked_file);
            }
        }
    }

    Ok(order)
}

/// Build the merged markdown document from a folder.
/// Returns the merged markdown string and a list of section boundaries (file_name, start_line).
pub fn build_merged_document(folder_path: &str) -> Result<(String, Vec<(String, usize)>), String> {
    let file_order = resolve_wikilinks(folder_path)?;
    let folder = Path::new(folder_path);

    let mut merged = String::new();
    let mut boundaries: Vec<(String, usize)> = Vec::new();

    for (idx, file_name) in file_order.iter().enumerate() {
        let file_path = folder.join(file_name);
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read '{}': {}", file_path.display(), e))?;

        // Process wikilinks in the content: replace resolved ones with inline references
        // and broken ones with placeholder text
        let processed = process_wikilinks_in_content(&content, folder, &file_name);

        let start_line = merged.lines().count();
        boundaries.push((file_name.clone(), start_line));

        if idx > 0 {
            merged.push_str("\n\n---\n\n");
        }
        merged.push_str(&format!("<!-- file: {} -->\n\n", file_name));
        merged.push_str(&processed);
    }

    Ok((merged, boundaries))
}

/// Find the entry point file in a folder.
/// Looks for `index.md`, then `main.md`. Returns error if neither exists.
fn find_entry_point(folder: &Path) -> Result<String, String> {
    for candidate in &["index.md", "main.md"] {
        if folder.join(candidate).exists() {
            return Ok(candidate.to_string());
        }
    }
    Err(
        "No entry point found. Please name your main file index.md or main.md.".to_string(),
    )
}

/// Extract all wikilink targets from markdown content.
fn extract_wikilinks(content: &str) -> Vec<String> {
    WIKILINK_RE
        .captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
        .collect()
}

/// Normalize a wikilink target to a filename.
/// Adds `.md` extension if missing.
fn normalize_link(link: &str) -> String {
    let trimmed = link.trim();
    if trimmed.ends_with(".md") {
        trimmed.to_string()
    } else {
        format!("{}.md", trimmed)
    }
}

/// Process wikilinks in content: replace resolved links with markdown internal links
/// and broken links with placeholder emphasis text.
fn process_wikilinks_in_content(
    content: &str,
    folder: &Path,
    _current_file: &str,
) -> String {
    WIKILINK_RE
        .replace_all(content, |caps: &regex::Captures| {
            let link_target = caps.get(1).unwrap().as_str().trim();
            let linked_file = normalize_link(link_target);
            let linked_path = folder.join(&linked_file);

            if linked_path.exists() {
                // Resolved link: replace with a markdown internal link to an anchor
                let anchor = linked_file.replace(".md", "");
                format!("[{}](#section-{})", link_target, anchor)
            } else {
                // Broken link: markdown emphasis placeholder
                format!("*Missing link: {}*", link_target)
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wikilinks() {
        let content = "See [[intro]] and [[conclusion.md]] for details.";
        let links = extract_wikilinks(content);
        assert_eq!(links, vec!["intro", "conclusion.md"]);
    }

    #[test]
    fn test_normalize_link() {
        assert_eq!(normalize_link("intro"), "intro.md");
        assert_eq!(normalize_link("intro.md"), "intro.md");
    }
}
