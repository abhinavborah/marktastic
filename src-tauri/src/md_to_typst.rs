use pulldown_cmark::{Event, Parser, Tag, TagEnd, CodeBlockKind, Alignment};

pub fn convert_md_to_typst(markdown: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut in_table = false;
    let mut table_header: Vec<String> = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut table_alignments: Vec<Alignment> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut in_row = false;
    let mut in_task_list = false;
    let mut task_checked = false;
    let mut list_stack: Vec<(bool, u64)> = Vec::new(); // (ordered, start_num)
    let mut in_link = false;
    let mut link_url = String::new();
    let mut link_text = String::new();
    let mut in_image = false;
    let mut image_src = String::new();
    let mut image_alt = String::new();
    let mut quote_depth = 0;

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        if quote_depth == 0 {
                            // paragraphs are implicit in typst
                        }
                    }
                    Tag::Heading { level, .. } => {
                        let eqs = "=".repeat(level as usize);
                        output.push_str(&format!("{} ", eqs));
                    }
                    Tag::BlockQuote(_) => {
                        quote_depth += 1;
                        output.push_str("#quote[\n");
                    }
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_content.clear();
                        if let CodeBlockKind::Fenced(lang) = kind {
                            code_lang = lang.to_string();
                        }
                    }
                    Tag::List(start_num) => {
                        list_stack.push((start_num.is_some(), start_num.unwrap_or(1)));
                    }
                    Tag::Item => {
                        let (ordered, _start) = list_stack.last().copied().unwrap_or((false, 1));
                        if ordered {
                            output.push_str(&format!("+ "));
                        } else {
                            output.push_str("- ");
                        }
                        if in_task_list {
                            if task_checked {
                                output.push_str("☑ ");
                            } else {
                                output.push_str("☐ ");
                            }
                        }
                    }
                    Tag::Emphasis => output.push_str("_"),
                    Tag::Strong => output.push_str("*"),
                    Tag::Strikethrough => output.push_str("#strike["),
                    Tag::Link { dest_url, .. } => {
                        in_link = true;
                        link_url = dest_url.to_string();
                        link_text.clear();
                    }
                    Tag::Image { dest_url, .. } => {
                        in_image = true;
                        image_src = dest_url.to_string();
                        image_alt.clear();
                    }
                    Tag::Table(alignments) => {
                        in_table = true;
                        table_alignments = alignments.to_vec();
                        table_header.clear();
                        table_rows.clear();
                    }
                    Tag::TableHead => {
                        in_row = true;
                        current_row.clear();
                    }
                    Tag::TableRow => {
                        in_row = true;
                        current_row.clear();
                    }
                    Tag::TableCell => {
                        // cell content starts
                    }
                    _ => {}
                }
            }
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::Paragraph => {
                        if quote_depth == 0 {
                            output.push_str("\n\n");
                        }
                    }
                    TagEnd::Heading(_) => {
                        output.push_str("\n\n");
                    }
                    TagEnd::BlockQuote(_) => {
                        quote_depth -= 1;
                        output.push_str("\n]\n\n");
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        if !code_lang.is_empty() {
                            output.push_str(&format!(
                                "#raw(block: true, lang: \"{}\", \"{}\")\n\n",
                                escape_string(&code_lang),
                                escape_string(&code_content.trim_end())
                            ));
                        } else {
                            output.push_str(&format!(
                                "#raw(block: true, \"{}\")\n\n",
                                escape_string(&code_content.trim_end())
                            ));
                        }
                        code_lang.clear();
                    }
                    TagEnd::List(..) => {
                        list_stack.pop();
                        if !list_stack.is_empty() {
                            // nested list, don't add extra newline
                        } else {
                            output.push_str("\n");
                        }
                    }
                    TagEnd::Item => {
                        output.push_str("\n");
                    }
                    TagEnd::Emphasis => output.push_str("_"),
                    TagEnd::Strong => output.push_str("*"),
                    TagEnd::Strikethrough => output.push_str("]"),
                    TagEnd::Link => {
                        in_link = false;
                        let text = link_text.trim();
                        if link_url.starts_with("#") {
                            // internal anchor
                            output.push_str(&format!("#link(\"{}\")[{}]", escape_string(&link_url), text));
                        } else {
                            output.push_str(&format!(
                                "#link(\"{}\")[{}]",
                                escape_string(&link_url),
                                text
                            ));
                        }
                    }
                    TagEnd::Image => {
                        in_image = false;
                        output.push_str(&format!(
                            "#image(\"{}\", alt: \"{}\")\n\n",
                            escape_string(&image_src),
                            escape_string(&image_alt)
                        ));
                    }
                    TagEnd::Table => {
                        in_table = false;
                        output.push_str("#table(\n");
                        // columns
                        let col_count = table_alignments.len();
                        output.push_str(&format!("  columns: {},\n", col_count));
                        // alignments
                        if !table_alignments.is_empty() {
                            let aligns: Vec<String> = table_alignments.iter().map(|a| {
                                match a {
                                    Alignment::Left => "left".to_string(),
                                    Alignment::Center => "center".to_string(),
                                    Alignment::Right => "right".to_string(),
                                    Alignment::None => "auto".to_string(),
                                }
                            }).collect();
                            output.push_str(&format!("  align: ({}),\n", aligns.join(", ")));
                        }
                        // header row
                        output.push_str("  table.header(\n");
                        for cell in &table_header {
                            output.push_str(&format!("    [{}],\n", cell.trim()));
                        }
                        output.push_str("  ),\n");
                        // data rows
                        for row in &table_rows {
                            for cell in row {
                                output.push_str(&format!("  [{}],\n", cell.trim()));
                            }
                        }
                        output.push_str(")\n\n");
                    }
                    TagEnd::TableHead => {
                        in_row = false;
                        table_header = current_row.clone();
                        current_row.clear();
                    }
                    TagEnd::TableRow => {
                        in_row = false;
                        table_rows.push(current_row.clone());
                        current_row.clear();
                    }
                    TagEnd::TableCell => {
                        // nothing special needed, content already added
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else if in_link {
                    link_text.push_str(&text);
                } else if in_image {
                    image_alt.push_str(&text);
                } else if in_table && in_row {
                    current_row.push(text.to_string());
                } else {
                    output.push_str(&escape_text(&text));
                }
            }
            Event::Code(code) => {
                output.push_str(&format!("`{}`", escape_string(&code)));
            }
            Event::Html(html) => {
                // ignore raw HTML for now
                output.push_str(&format!("/* raw HTML omitted: {} */", html));
            }
            Event::FootnoteReference(name) => {
                output.push_str(&format!("#footnote[{}]", name));
            }
            Event::SoftBreak | Event::HardBreak => {
                if in_code_block {
                    code_content.push('\n');
                } else if in_table && in_row {
                    // don't add breaks in table cells
                } else {
                    output.push('\n');
                }
            }
            Event::Rule => {
                output.push_str("#line(length: 100%, stroke: 0.5pt + gray)\n\n");
            }
            Event::TaskListMarker(checked) => {
                in_task_list = true;
                task_checked = checked;
            }
            _ => {}
        }
    }

    output.trim().to_string()
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
}

fn escape_text(s: &str) -> String {
    // In Typst content mode, we need to escape certain characters
    s.replace("\\", "\\ ")
        .replace("#", "\\#")
        .replace("*", "\\*")
        .replace("_", "\\_")
        .replace("`", "\\`")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("@", "\\@")
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("$", "\\$")
}
