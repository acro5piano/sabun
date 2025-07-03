use std::io::{self, Write};
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::MoveTo,
};
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
        if atty::is(atty::Stream::Stdout) && diff_lines.len() > 20 {
            self.display_interactive(diff_lines)
        } else {
            self.display_direct(diff_lines)
        }
    }
    
    fn display_direct(&mut self, diff_lines: &[DiffLine]) -> Result<()> {
        let mut buffer = self.buffer_writer.buffer();
        
        for line in diff_lines {
            self.write_line(&mut buffer, line)?;
        }
        
        self.buffer_writer.print(&buffer)?;
        Ok(())
    }
    
    fn display_interactive(&mut self, diff_lines: &[DiffLine]) -> Result<()> {
        enable_raw_mode()?;
        
        let mut current_line = 0;
        
        loop {
            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
            
            let term_size = crossterm::terminal::size()?;
            let max_lines = (term_size.1 as usize).saturating_sub(2);
            
            let mut buffer = self.buffer_writer.buffer();
            let end_line = std::cmp::min(current_line + max_lines, diff_lines.len());
            
            for line in &diff_lines[current_line..end_line] {
                self.write_line_no_newline(&mut buffer, line)?;
                writeln!(buffer)?;
            }
            
            // Status line
            let progress = if diff_lines.is_empty() {
                "100%".to_string()
            } else {
                format!("{}%", (end_line * 100) / diff_lines.len())
            };
            
            buffer.set_color(&ColorSpec::new()
                .set_fg(Some(termcolor::Color::Black))
                .set_bg(Some(termcolor::Color::White))
                .set_bold(true))?;
            write!(buffer, " {} lines {}-{} of {} ({}) - q:quit, j/k:scroll ", 
                   diff_lines.len(), current_line + 1, end_line, diff_lines.len(), progress)?;
            buffer.reset()?;
            
            self.buffer_writer.print(&buffer)?;
            io::stdout().flush()?;
            
            match event::read()? {
                Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) |
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => break,
                
                Event::Key(KeyEvent { code: KeyCode::Char('j'), .. }) | 
                Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                    if end_line < diff_lines.len() {
                        current_line += 1;
                    }
                },
                Event::Key(KeyEvent { code: KeyCode::Char('k'), .. }) |
                Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                    if current_line > 0 {
                        current_line -= 1;
                    }
                },
                Event::Key(KeyEvent { code: KeyCode::Char('d'), .. }) => {
                    current_line = std::cmp::min(current_line + max_lines / 2, 
                                                diff_lines.len().saturating_sub(max_lines));
                },
                Event::Key(KeyEvent { code: KeyCode::Char('u'), .. }) => {
                    current_line = current_line.saturating_sub(max_lines / 2);
                },
                Event::Key(KeyEvent { code: KeyCode::Char('f'), .. }) |
                Event::Key(KeyEvent { code: KeyCode::PageDown, .. }) |
                Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) => {
                    if end_line < diff_lines.len() {
                        current_line = std::cmp::min(current_line + max_lines, 
                                                    diff_lines.len().saturating_sub(max_lines));
                    }
                },
                Event::Key(KeyEvent { code: KeyCode::Char('b'), .. }) |
                Event::Key(KeyEvent { code: KeyCode::PageUp, .. }) => {
                    current_line = current_line.saturating_sub(max_lines);
                },
                Event::Key(KeyEvent { code: KeyCode::Char('g'), .. }) => {
                    current_line = 0;
                },
                Event::Key(KeyEvent { code: KeyCode::Char('G'), .. }) => {
                    current_line = diff_lines.len().saturating_sub(max_lines);
                },
                _ => {},
            }
        }
        
        disable_raw_mode()?;
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
                if let Some(line_no) = line.new_line_number {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, "+")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::added_line())?;
                writeln!(buffer)?;
            },
            DiffLineType::Removed => {
                buffer.set_color(&DarkTheme::removed_line())?;
                if let Some(line_no) = line.old_line_number {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, "-")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::removed_line())?;
                writeln!(buffer)?;
            },
            DiffLineType::Context => {
                buffer.set_color(&DarkTheme::context_line())?;
                if let Some(line_no) = line.old_line_number.or(line.new_line_number) {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, " ")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::context_line())?;
                writeln!(buffer)?;
            },
        }
        
        buffer.reset()?;
        Ok(())
    }
    
    fn write_line_no_newline(&self, buffer: &mut termcolor::Buffer, line: &DiffLine) -> Result<()> {
        match line.line_type {
            DiffLineType::FileHeader => {
                buffer.set_color(&DarkTheme::file_header())?;
                write!(buffer, "{}", line.content)?;
            },
            DiffLineType::HunkHeader => {
                buffer.set_color(&DarkTheme::hunk_header())?;
                write!(buffer, "{}", line.content)?;
            },
            DiffLineType::Added => {
                buffer.set_color(&DarkTheme::added_line())?;
                if let Some(line_no) = line.new_line_number {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, "+")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::added_line())?;
            },
            DiffLineType::Removed => {
                buffer.set_color(&DarkTheme::removed_line())?;
                if let Some(line_no) = line.old_line_number {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, "-")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::removed_line())?;
            },
            DiffLineType::Context => {
                buffer.set_color(&DarkTheme::context_line())?;
                if let Some(line_no) = line.old_line_number.or(line.new_line_number) {
                    write!(buffer, "{:>6} ", line_no)?;
                } else {
                    write!(buffer, "       ")?;
                }
                write!(buffer, " ")?;
                self.write_syntax_highlighted(buffer, &line.syntax_highlights, &DarkTheme::context_line())?;
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
                SyntaxType::Function => {
                    color_spec.set_fg(Some(termcolor::Color::Blue)).set_intense(true);
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