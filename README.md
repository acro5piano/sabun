# sabun

A simple diff tool with syntax highlighting optimized for dark terminal backgrounds.

## Features

- **Dual input modes**: Compare files directly or process diff from STDIN
- **Syntax highlighting**: Keywords, strings, comments, numbers with dark theme colors
- **Background coloring**: Red/green backgrounds for removed/added lines that extend full terminal width
- **Git integration**: Works as git pager and diffFilter
- **External pager support**: Outputs to stdout for use with `less` or other pagers

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

### Use with external pager
```bash
sabun file1.rs file2.rs | less -R
git diff | sabun | less -R
```

### Git integration
Add to your `~/.gitconfig`:
```ini
[core]
    pager = sabun | less -R

[interactive]
    diffFilter = sabun
```

## Color Scheme

Optimized for dark terminals:
- **File headers**: Bright white, bold
- **Added lines**: Dark green background (full width) with syntax highlighting
- **Removed lines**: Dark red background (full width) with syntax highlighting  
- **Context lines**: Regular white with syntax highlighting
- **Hunk headers**: Cyan, bold
- **Keywords**: Blue, bold
- **Strings**: Bright green
- **Comments**: Dimmed green
- **Numbers**: Bright yellow
- **Types**: Bright cyan

## Examples

```bash
# Compare Rust files
sabun src/main.rs src/lib.rs

# Use with git and external pager
git show HEAD | sabun | less -R

# Pipe any diff through sabun
diff -u old.txt new.txt | sabun | less -R

# Direct output (no pager)
git diff | sabun
```
