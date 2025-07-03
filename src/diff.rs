use similar::{ChangeTag, TextDiff};
use anyhow::Result;

use crate::syntax::{SyntaxHighlighter, SyntaxType};

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub old_line_number: Option<usize>,
    pub new_line_number: Option<usize>,
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
            old_line_number: None,
            new_line_number: None,
            content: format!("--- {}", old_name),
            syntax_highlights: vec![(SyntaxType::Normal, format!("--- {}", old_name))],
        });
        
        result.push(DiffLine {
            line_type: DiffLineType::FileHeader,
            old_line_number: None,
            new_line_number: None,
            content: format!("+++ {}", new_name),
            syntax_highlights: vec![(SyntaxType::Normal, format!("+++ {}", new_name))],
        });
        
        let diff = TextDiff::from_lines(old_content, new_content);
        let language = self.syntax_highlighter.detect_language(old_filename.or(new_filename));
        
        for (group_idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if group_idx > 0 {
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    old_line_number: None,
                    new_line_number: None,
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
                old_line_number: None,
                new_line_number: None,
                content: hunk_header.clone(),
                syntax_highlights: vec![(SyntaxType::Normal, hunk_header)],
            });
            
            let mut old_line_no = old_start;
            let mut new_line_no = new_start;
            
            for op in group {
                for change in diff.iter_changes(op) {
                    let (line_type, old_line_num, new_line_num) = match change.tag() {
                        ChangeTag::Delete => {
                            let result = (DiffLineType::Removed, Some(old_line_no), None);
                            old_line_no += 1;
                            result
                        },
                        ChangeTag::Insert => {
                            let result = (DiffLineType::Added, None, Some(new_line_no));
                            new_line_no += 1;
                            result
                        },
                        ChangeTag::Equal => {
                            let result = (DiffLineType::Context, Some(old_line_no), Some(new_line_no));
                            old_line_no += 1;
                            new_line_no += 1;
                            result
                        },
                    };
                    
                    let line_content = change.value().trim_end_matches('\n').to_string();
                    let syntax_highlights = self.syntax_highlighter
                        .highlight_line(&line_content, language)
                        .unwrap_or_else(|_| vec![(SyntaxType::Normal, line_content.clone())]);
                    
                    result.push(DiffLine {
                        line_type,
                        old_line_number: old_line_num,
                        new_line_number: new_line_num,
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
        let mut old_line_no = 0;
        let mut new_line_no = 0;
        
        for line in diff_content.lines() {
            if line.starts_with("--- ") {
                let filename = line.strip_prefix("--- ").unwrap_or("");
                if let Some(lang) = self.syntax_highlighter.detect_language(Some(filename)) {
                    current_language = Some(lang);
                }
                
                result.push(DiffLine {
                    line_type: DiffLineType::FileHeader,
                    old_line_number: None,
                    new_line_number: None,
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
                    old_line_number: None,
                    new_line_number: None,
                    content: line.to_string(),
                    syntax_highlights: vec![(SyntaxType::Normal, line.to_string())],
                });
            } else if line.starts_with("@@") {
                if let Some(captures) = parse_hunk_header(line) {
                    old_line_no = captures.0;
                    new_line_no = captures.1;
                }
                
                result.push(DiffLine {
                    line_type: DiffLineType::HunkHeader,
                    old_line_number: None,
                    new_line_number: None,
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
                    old_line_number: None,
                    new_line_number: Some(new_line_no),
                    content,
                    syntax_highlights,
                });
                new_line_no += 1;
            } else if line.starts_with('-') {
                let content = line.chars().skip(1).collect::<String>();
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(&content, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, content.clone())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    old_line_number: Some(old_line_no),
                    new_line_number: None,
                    content,
                    syntax_highlights,
                });
                old_line_no += 1;
            } else if line.starts_with(' ') {
                let content = line.chars().skip(1).collect::<String>();
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(&content, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, content.clone())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    old_line_number: Some(old_line_no),
                    new_line_number: Some(new_line_no),
                    content,
                    syntax_highlights,
                });
                old_line_no += 1;
                new_line_no += 1;
            } else {
                let syntax_highlights = self.syntax_highlighter
                    .highlight_line(line, current_language)
                    .unwrap_or_else(|_| vec![(SyntaxType::Normal, line.to_string())]);
                
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    old_line_number: None,
                    new_line_number: None,
                    content: line.to_string(),
                    syntax_highlights,
                });
            }
        }
        
        Ok(result)
    }
}

fn parse_hunk_header(line: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let old_part = parts[1].strip_prefix('-')?;
        let new_part = parts[2].strip_prefix('+')?;
        
        let old_line = old_part.split(',').next()?.parse().ok()?;
        let new_line = new_part.split(',').next()?.parse().ok()?;
        
        Some((old_line, new_line))
    } else {
        None
    }
}