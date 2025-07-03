# sabun

A simple diff tool with syntax highlighting optimized for dark terminal backgrounds.

## Features

- **Dual input modes**: Compare files directly or process diff from STDIN
- **Syntax highlighting**: Keywords, strings, comments, numbers with dark theme colors
- **Interactive pager**: Navigate through large diffs with vim-like keybindings
- **Git integration**: Works as git pager and diffFilter

## Installation

```bash
# Build the project
cargo build --release

# Or use the install script
./install.sh
```

## Usage

### Compare two files
```bash
sabun file1.rs file2.rs
```

### Process diff from STDIN
```bash
git diff | sabun
diff -u file1 file2 | sabun
```

### Git integration
Add to your `~/.gitconfig`:
```ini
[core]
    pager = sabun

[interactive]
    diffFilter = sabun
```

## Pager Controls

When viewing large diffs (>20 lines), sabun enters interactive mode:

- `j` / `↓` - Scroll down one line
- `k` / `↑` - Scroll up one line  
- `d` - Scroll down half page
- `u` - Scroll up half page
- `f` / `PgDn` / `Space` - Scroll down full page
- `b` / `PgUp` - Scroll up full page
- `g` - Go to beginning
- `G` - Go to end
- `q` / `Esc` - Quit

## Color Scheme

Optimized for dark terminals:
- **File headers**: Bright white, bold
- **Added lines**: Green text with dark green background
- **Removed lines**: Red text with dark red background
- **Context lines**: Regular white
- **Keywords**: Blue, bold
- **Strings**: Bright green
- **Comments**: Dimmed green
- **Numbers**: Bright yellow

## Examples

```bash
# Compare Rust files
sabun src/main.rs src/lib.rs

# Use with git
git show HEAD | sabun

# Pipe any diff
diff -u old.txt new.txt | sabun
```
