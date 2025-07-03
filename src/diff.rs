use similar::{ChangeTag, TextDiff};
use anyhow::Result;

use crate::syntax::{SyntaxHighlighter, SyntaxType};

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub syntax_highlights: Vec<(SyntaxType, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    Added,
    Removed,
    Context,
    FileHeader,
    HunkHeader,
}

pub struct DiffProcessor {
    syntax_highlighter: SyntaxHighlighter,
}

impl DiffProcessor {
    pub fn new() -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new().unwrap_or_else(|_| {
                eprintln!("Warning: Failed to initialize syntax highlighter");
                SyntaxHighlighter::new().unwrap()
            }),
        }
    }
    
    pub fn generate_diff(&self, old_content: &str, new_content: &str, old_filename: Option<&str>, new_filename: Option<&str>) -> Result<Vec<DiffLine>> {
        let mut result = Vec::new();
        
        let old_name = old_filename.unwrap_or("a");
        let new_name = new_filename.unwrap_or("b");
        
        result.push(DiffLine {
            line_type: DiffLineType::FileHeader,
            content: format!("--- {}", old_name),
            syntax_highlights: vec![(SyntaxType::Normal, format!("--- {}", old_name))],
        });
        
        result.push(DiffLine {
            line_type: DiffLineType::FileHeader,
            content: format!("+++ {}", new_name),
            syntax_highlights: vec![(SyntaxType::Normal, format!("+++ {}", new_name))],
        });
        
        let diff = TextDiff::from_lines(old_content, new_content);
        let language = self.syntax_highlighter.detect_language(old_filename.or(new_filename));
        
        for (group_idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if group_idx > 0 {
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    content: String::new(),
                    syntax_highlights: vec![(SyntaxType::Normal, String::new())],
                });
            }
            
            let first_op = &group[0];
            let last_op = &group[group.len() - 1];
            
            let old_start = first_op.old_range().start + 1;
            let old_end = last_op.old_range().end;
            let new_start = first_op.new_range().start + 1;
            let new_end = last_op.new_range().end;
            
            let hunk_header = format!("@@ -{},{} +{},{} @@", old_start, old_end - old_start + 1, new_start, new_end - new_start + 1);
            
            result.push(DiffLine {
                line_type: DiffLineType::HunkHeader,
                content: hunk_header.clone(),
                syntax_highlights: vec![(SyntaxType::Normal, hunk_header)],
            });
            
            let mut _old_line_no = old_start;
            let mut _new_line_no = new_start;
            
            for op in group {
                for change in diff.iter_changes(op) {
                    let line_type = match change.tag() {
                        ChangeTag::Delete => {
                            _old_line_no += 1;
                            DiffLineType::Removed
                        },
                        ChangeTag::Insert => {
                            _new_line_no += 1;
                            DiffLineType::Added
                        },
                        ChangeTag::Equal => {
                            _old_line_no += 1;
                            _new_line_no += 1;
                            DiffLineType::Context
                        },
                    };
                    
                    let line_content = change.value().trim_end_matches('\n').to_string();
                    let syntax_highlights = self.syntax_highlighter
                        .highlight_line(&line_content, language)
                        .unwrap_or_else(|_| vec![(SyntaxType::Normal, line_content.clone())]);
                    
                    result.push(DiffLine {
                        line_type,
                        content: line_content,
                        syntax_highlights,
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    pub fn parse_diff(&self, diff_content: &str) -> Result<Vec<DiffLine>> {
        let mut result = Vec::new();
        let mut current_language = None;
        
        for line in diff_content.lines() {
            if line.starts_with("--- ") {
                let filename = line.strip_prefix("--- ").unwrap_or("");
                if let Some(lang) = self.syntax_highlighter.detect_language(Some(filename)) {
                    current_language = Some(lang);
                }
                
                result.push(DiffLine {
                    line_type: DiffLineType::FileHeader,
                    content: line.to_string(),
                    syntax_highlights: vec![(SyntaxType::Normal, line.to_string())],
                });
            } else if line.starts_with("+++ ") {
                let filename = line.strip_prefix("+++ ").unwrap_or("");
                if current_language.is_none() {
                    if let Some(lang) = self.syntax_highlighter.detect_language(Some(filename)) {
                        current_language = Some(lang);
                    }
                }
                
                result.push(DiffLine {
                    line_type: DiffLineType::FileHeader,
                    content: line.to_string(),
                    syntax_highlights: vec![(SyntaxType::Normal, line.to_string())],
                });
            } else if line.starts_with("@@") {
                
                result.push(DiffLine {
                    line_type: DiffLineType::HunkHeader,
                    content: line.to_string(),
                    syntax_highlights: vec![(SyntaxType::Normal, line.to_string())],
                });
            } else if line.starts_with('+') {
                let content = line.chars().skip(1).collect::<String>();
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(&content, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, content.clone())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Added,
                    content,
                    syntax_highlights,
                });
            } else if line.starts_with('-') {
                let content = line.chars().skip(1).collect::<String>();
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(&content, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, content.clone())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    content,
                    syntax_highlights,
                });
            } else if line.starts_with(' ') {
                let content = line.chars().skip(1).collect::<String>();
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(&content, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, content.clone())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    content,
                    syntax_highlights,
                });
            } else {
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(line, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, line.to_string())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    content: line.to_string(),
                    syntax_highlights,
                });
            }
        }
        
        Ok(result)
    }
}

