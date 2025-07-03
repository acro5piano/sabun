use termcolor::{Color, ColorSpec};

pub struct DarkTheme;

impl DarkTheme {
    pub fn file_header() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::White))
            .set_bold(true)
            .set_intense(true);
        spec
    }
    
    pub fn line_number() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Cyan))
            .set_dimmed(true);
        spec
    }
    
    pub fn added_line() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Green))
            .set_bg(Some(Color::Rgb(0, 64, 0)));
        spec
    }
    
    pub fn removed_line() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Red))
            .set_bg(Some(Color::Rgb(64, 0, 0)));
        spec
    }
    
    pub fn context_line() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::White));
        spec
    }
    
    pub fn hunk_header() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Cyan))
            .set_bold(true);
        spec
    }
    
    pub fn keyword() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Blue))
            .set_intense(true);
        spec
    }
    
    pub fn string() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Green))
            .set_intense(true);
        spec
    }
    
    pub fn comment() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Green))
            .set_dimmed(true);
        spec
    }
    
    pub fn number() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Yellow))
            .set_intense(true);
        spec
    }
    
    pub fn function() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Blue))
            .set_intense(true);
        spec
    }
    
    pub fn type_name() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Cyan))
            .set_intense(true);
        spec
    }
}