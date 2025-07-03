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
    
    pub fn added_line() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_bg(Some(Color::Rgb(0, 40, 0))); // Dark green background
        spec
    }
    
    pub fn removed_line() -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_bg(Some(Color::Rgb(40, 0, 0))); // Dark red background
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
    
}