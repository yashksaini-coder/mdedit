# 0.6.3

* wrong Cargo.toml

# 0.6.2

* upgrade dialogs to rat-dialog. nice to use.

# 0.6.1

* fix: strange locale in WSL. fallback

# 0.6.0

* fix: running mdedit with a directory failed panicky.
  now uses this as the base directory and shows it's files.
* fix: update to new splitter

# 0.5.7

* fix: use of deprecated

# 0.5.6

* fix: published the wrong cargo.toml

# 0.5.5

* feature: start with different doc-types.
    - md - markdown support enabled
    - txt - plain text editing
    - future: \[rs\] - edit markdown in .rs comments
* fix: startup focus
* fix: use new TextArea::text_style_map()
* fix: cursor position didn't update on moves

# 0.5.4

* feature: add Black&White theme

# 0.5.3

* fix: switch to rat-theme2

# 0.5.1

* fix: event-handling of file-dialog + error-dialog was
  the wrong way around.

# 0.5.0

* feature: add 'file_pattern' and 'log' level to config.
* feature: load initial tree in background.

# 0.4.0

* feature: use ignore crate to build the current tree.
  respecting .gitignore & co sounds like a good feature.

# 0.3.0

* a lot of changes ...

# 0.2.0

* copy from rat-salsa example
