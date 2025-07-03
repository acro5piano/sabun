use std::io::Write;
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};
use anyhow::Result;

use crate::diff::{DiffLine, DiffLineType};
use crate::syntax::SyntaxType;
use crate::colors::DarkTheme;

pub struct Pager {
    buffer_writer: BufferWriter,
}

impl Pager {
    pub fn new() -> Self {
        let buffer_writer = BufferWriter::stdout(ColorChoice::Auto);
        Self { buffer_writer }
    }
    
    pub fn display(&mut self, diff_lines: &[DiffLine]) -> Result<()> {
        self.display_direct(diff_lines)
    }
    
    fn display_direct(&mut self, diff_lines: &[DiffLine]) -> Result<()> {
        let mut buffer = self.buffer_writer.buffer();
        
        for line in diff_lines {
            self.write_line(&mut buffer, line)?;
        }
        
        self.buffer_writer.print(&buffer)?;
        Ok(())
    }
    
    
    fn write_line(&self, buffer: &mut termcolor::Buffer, line: &DiffLine) -> Result<()> {
        match line.line_type {
            DiffLineType::FileHeader => {
                buffer.set_color(&DarkTheme::file_header())?;
                writeln!(buffer, "{}", line.content)?;
            },
            DiffLineType::HunkHeader => {
                buffer.set_color(&DarkTheme::hunk_header())?;
                writeln!(buffer, "{}", line.content)?;
            },
            DiffLineType::Added => {
                buffer.set_color(&DarkTheme::added_line())?;
                write!(buffer, "+")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::added_line())?;
                // Clear to end of line with current background color
                write!(buffer, "\x1b[K")?;
                writeln!(buffer)?;
            },
            DiffLineType::Removed => {
                buffer.set_color(&DarkTheme::removed_line())?;
                write!(buffer, "-")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::removed_line())?;
                // Clear to end of line with current background color
                write!(buffer, "\x1b[K")?;
                writeln!(buffer)?;
            },
            DiffLineType::Context => {
                buffer.set_color(&DarkTheme::context_line())?;
                write!(buffer, " ")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::context_line())?;
                writeln!(buffer)?;
            },
        }
        
        buffer.reset()?;
        Ok(())
    }
    
    
    fn write_syntax_highlighted(&self, buffer: &mut termcolor::Buffer, highlights: &[(SyntaxType, String)], base_color: &ColorSpec) -> Result<()> {
        for (syntax_type, text) in highlights {
            let mut color_spec = base_color.clone();
            
            match syntax_type {
                SyntaxType::Keyword => {
                    color_spec.set_fg(Some(termcolor::Color::Blue)).set_bold(true);
                },
                SyntaxType::String => {
                    color_spec.set_fg(Some(termcolor::Color::Green)).set_intense(true);
                },
                SyntaxType::Comment => {
                    color_spec.set_fg(Some(termcolor::Color::Green)).set_dimmed(true);
                },
                SyntaxType::Number => {
                    color_spec.set_fg(Some(termcolor::Color::Yellow)).set_intense(true);
                },
                SyntaxType::Type => {
                    color_spec.set_fg(Some(termcolor::Color::Cyan)).set_intense(true);
                },
                SyntaxType::Normal => {
                    // Keep base color
                },
            }
            
            buffer.set_color(&color_spec)?;
            write!(buffer, "{}", text)?;
        }
        Ok(())
    }
}