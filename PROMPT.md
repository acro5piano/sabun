you are creating a tool called `sabun`, which

- is yet simple another diff + syntax highlighing tool
- written in Rust
- uses Treesitter for syntax highlighting
- pager included
- File header will be brighten
- Optimized for dark background color for simplicity
- Just add green (added) and red (removed) background color.
- See ./reference.png for other facial references

CLI usage:

```
# 1. two files
sabun file_a file_b

# 2. from STDIN
cat some.diff | sabun
```

and ideally we can configure .gitconfig will be like this:

```
[core]
    pager = sabun

[interactive]
    diffFilter = sabun
```
