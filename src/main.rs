use clap::{Arg, Command};
use std::io::{self, Read, IsTerminal};
use std::fs;
use anyhow::Result;

mod diff;
mod syntax;
mod pager;
mod colors;

use diff::DiffProcessor;
use pager::Pager;

fn main() -> Result<()> {
    let matches = Command::new("sabun")
        .version("0.1.0")
        .about("A simple diff tool with syntax highlighting")
        .arg(
            Arg::new("file1")
                .help("First file to compare")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("file2")
                .help("Second file to compare")
                .index(2)
                .required(false)
        )
        .get_matches();

    let processor = DiffProcessor::new();
    
    if let (Some(file1), Some(file2)) = (matches.get_one::<String>("file1"), matches.get_one::<String>("file2")) {
        let content1 = fs::read_to_string(file1)?;
        let content2 = fs::read_to_string(file2)?;
        let diff_output = processor.generate_diff(&content1, &content2, Some(file1), Some(file2))?;
        
        let mut pager = Pager::new();
        pager.display(&diff_output)?;
    } else {
        if !io::stdin().is_terminal() {
            let mut stdin_content = String::new();
            io::stdin().read_to_string(&mut stdin_content)?;
            let diff_output = processor.parse_diff(&stdin_content)?;
            
            let mut pager = Pager::new();
            pager.display(&diff_output)?;
        } else {
            eprintln!("Usage: sabun <file1> <file2> or pipe diff to stdin");
            std::process::exit(1);
        }
    }
    
    Ok(())
}