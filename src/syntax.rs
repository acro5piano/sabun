use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxType {
    Keyword,
    String,
    Comment,
    Number,
    Function,
    Type,
    Normal,
}

pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    pub fn detect_language<'a>(&self, filename: Option<&'a str>) -> Option<&'a str> {
        filename.and_then(|f| {
            Path::new(f)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| match ext {
                    "rs" => "rust",
                    "js" | "javascript" => "javascript", 
                    "py" | "python" => "python",
                    "c" => "c",
                    "json" => "json",
                    _ => ext,
                })
        })
    }
    
    pub fn highlight_line(&self, line: &str, _language: Option<&str>) -> Result<Vec<(SyntaxType, String)>> {
        Ok(self.basic_highlight(line))
    }
    
    fn basic_highlight(&self, line: &str) -> Vec<(SyntaxType, String)> {
        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_char = None;
        let mut in_comment = false;
        let mut chars = line.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if in_comment {
                let rest_of_line = std::iter::once(ch).chain(chars).collect::<String>();
                result.push((SyntaxType::Comment, rest_of_line));
                break;
            }
            
            if in_string {
                current_word.push(ch);
                if Some(ch) == string_char && chars.peek() != Some(&'\\') {
                    result.push((SyntaxType::String, current_word.clone()));
                    current_word.clear();
                    in_string = false;
                    string_char = None;
                }
                continue;
            }
            
            match ch {
                '"' | '\'' => {
                    if !current_word.is_empty() {
                        result.push((self.classify_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    current_word.push(ch);
                    in_string = true;
                    string_char = Some(ch);
                }
                '/' if chars.peek() == Some(&'/') => {
                    if !current_word.is_empty() {
                        result.push((self.classify_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    current_word.push(ch);
                    current_word.push(chars.next().unwrap());
                    in_comment = true;
                }
                '#' => {
                    if !current_word.is_empty() {
                        result.push((self.classify_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    current_word.push(ch);
                    in_comment = true;
                }
                c if c.is_whitespace() || c.is_ascii_punctuation() => {
                    if !current_word.is_empty() {
                        result.push((self.classify_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    if !c.is_whitespace() {
                        result.push((SyntaxType::Normal, c.to_string()));
                    } else {
                        result.push((SyntaxType::Normal, c.to_string()));
                    }
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }
        
        if !current_word.is_empty() {
            if in_comment {
                result.push((SyntaxType::Comment, current_word));
            } else if in_string {
                result.push((SyntaxType::String, current_word));
            } else {
                result.push((self.classify_word(&current_word), current_word));
            }
        }
        
        if result.is_empty() {
            result.push((SyntaxType::Normal, line.to_string()));
        }
        
        result
    }
    
    fn classify_word(&self, word: &str) -> SyntaxType {
        match word {
            "fn" | "let" | "mut" | "const" | "static" | "struct" | "enum" | "impl" | "trait" | 
            "use" | "pub" | "mod" | "if" | "else" | "match" | "while" | "for" | "loop" | 
            "break" | "continue" | "return" | "true" | "false" | "null" | "undefined" |
            "function" | "var" | "class" | "def" | "import" | "from" | "as" => SyntaxType::Keyword,
            
            w if w.chars().all(|c| c.is_ascii_digit()) => SyntaxType::Number,
            w if w.parse::<f64>().is_ok() => SyntaxType::Number,
            
            w if w.chars().next().map_or(false, |c| c.is_uppercase()) => SyntaxType::Type,
            
            _ => SyntaxType::Normal,
        }
    }
}