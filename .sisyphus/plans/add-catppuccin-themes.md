# Add Catppuccin Themes

## TL;DR

> **Quick Summary**: Add "Catppuccin Latte" (Light) and "Catppuccin Frappe" (Dark) themes to Ekphos, including them in the default distribution.
> 
> **Deliverables**:
> - `themes/catppuccin-latte.toml`
> - `themes/catppuccin-frappe.toml`
> - Update to `src/config.rs` to deploy these themes
> 
> **Estimated Effort**: Short
> **Parallel Execution**: YES - 1 wave
> **Critical Path**: Create files → Update config

---

## Context

### Original Request
Add Catppuccin Latte and Frappe themes, referencing standard Catppuccin palettes.

### Interview Summary
- **Colors**: Sourced from standard Catppuccin palette.
- **Mapping**: Mapped to Ekphos TOML structure (base, accent, semantic, ui).
- **Distribution**: Must be added to `src/config.rs` to ensure users get them on startup.

---

## Work Objectives

### Core Objective
Enable users to select Catppuccin Latte and Frappe themes.

### Concrete Deliverables
- `themes/catppuccin-latte.toml`
- `themes/catppuccin-frappe.toml`
- Modified `src/config.rs`

### Definition of Done
- [ ] Theme files exist in `themes/`
- [ ] `src/config.rs` contains logic to write these files to `~/.config/ekphos/themes/` if missing
- [ ] Themes appear in Command Palette (Ctrl+P)
- [ ] Themes look correct when applied

---

## Verification Strategy

### Automated Verification
None (Visual/Config task).

### Manual Verification
1. **Build and Run**: `cargo run`
2. **Open Command Palette**: `Ctrl+p`
3. **Filter**: Type "Latte" or "Frappe"
4. **Select**: Choose "Catppuccin Latte"
5. **Verify**: UI switches to light theme, colors match Catppuccin Latte.
6. **Repeat**: Switch to "Catppuccin Frappe", verify dark theme colors.
7. **Config Check**: Verify files exist in `~/.config/ekphos/themes/`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Create Latte Theme
├── Task 2: Create Frappe Theme
└── Task 3: Register Themes in Config
```

---

## TODOs

- [ ] 1. Create Catppuccin Latte Theme File

  **What to do**:
  - Create `themes/catppuccin-latte.toml` with the content below.

  **Content**:
  ```toml
  # Catppuccin Latte
  # A light theme for Ekphos

  [base]
  background = "#eff1f5"
  background_secondary = "#e6e9ef"
  foreground = "#4c4f69"
  muted = "#6c6f85"

  [accent]
  primary = "#1e66f5"
  secondary = "#ea76cb"

  [semantic]
  error = "#d20f39"
  warning = "#df8e1d"
  success = "#40a02b"
  info = "#04a5e5"

  [ui]
  border = "#acb0be"
  border_focused = "#1e66f5"
  selection = "#ccd0da"
  cursor = "#dc8a78"

  [ui.statusbar]
  background = "#e6e9ef"
  foreground = "#4c4f69"
  brand = "#1e66f5"
  mode = "#6c6f85"
  separator = "#acb0be"

  [ui.dialog]
  background = "#eff1f5"
  border = "#1e66f5"
  title = "#1e66f5"
  text = "#4c4f69"

  [ui.sidebar]
  background = "#e6e9ef"
  item = "#4c4f69"
  item_selected = "#fe640b"
  folder = "#179299"
  folder_expanded = "#04a5e5"

  [ui.content]
  background = "#eff1f5"
  text = "#4c4f69"
  heading1 = "#1e66f5"
  heading2 = "#40a02b"
  heading3 = "#df8e1d"
  heading4 = "#8839ef"
  link = "#04a5e5"
  link_invalid = "#d20f39"
  code = "#40a02b"
  code_background = "#e6e9ef"
  blockquote = "#6c6f85"
  list_marker = "#8839ef"

  [ui.outline]
  background = "#e6e9ef"
  heading1 = "#1e66f5"
  heading2 = "#40a02b"
  heading3 = "#df8e1d"
  heading4 = "#8839ef"

  [ui.search]
  background = "#e6e9ef"
  border = "#1e66f5"
  input = "#4c4f69"
  match_highlight = "#fe640b"
  match_current = "#d20f39"
  match_count = "#6c6f85"

  [ui.editor]
  heading1 = "#1e66f5"
  heading2 = "#40a02b"
  heading3 = "#df8e1d"
  heading4 = "#8839ef"
  heading5 = "#04a5e5"
  heading6 = "#6c6f85"
  code = "#40a02b"
  link = "#04a5e5"
  blockquote = "#6c6f85"
  list_marker = "#8839ef"
  bold = "#df8e1d"
  italic = "#179299"
  ```

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Acceptance Criteria**:
  - File exists at `themes/catppuccin-latte.toml`
  - Content matches exactly

- [ ] 2. Create Catppuccin Frappe Theme File

  **What to do**:
  - Create `themes/catppuccin-frappe.toml` with the content below.

  **Content**:
  ```toml
  # Catppuccin Frappe
  # A dark theme for Ekphos

  [base]
  background = "#303446"
  background_secondary = "#292c3c"
  foreground = "#c6d0f5"
  muted = "#a5adce"

  [accent]
  primary = "#8caaee"
  secondary = "#f4b8e4"

  [semantic]
  error = "#e78284"
  warning = "#e5c890"
  success = "#a6d189"
  info = "#99d1db"

  [ui]
  border = "#626880"
  border_focused = "#8caaee"
  selection = "#51576d"
  cursor = "#f2d5cf"

  [ui.statusbar]
  background = "#292c3c"
  foreground = "#c6d0f5"
  brand = "#8caaee"
  mode = "#a5adce"
  separator = "#626880"

  [ui.dialog]
  background = "#303446"
  border = "#8caaee"
  title = "#8caaee"
  text = "#c6d0f5"

  [ui.sidebar]
  background = "#292c3c"
  item = "#c6d0f5"
  item_selected = "#ef9f76"
  folder = "#81c8be"
  folder_expanded = "#99d1db"

  [ui.content]
  background = "#303446"
  text = "#c6d0f5"
  heading1 = "#8caaee"
  heading2 = "#a6d189"
  heading3 = "#e5c890"
  heading4 = "#ca9ee6"
  link = "#99d1db"
  link_invalid = "#e78284"
  code = "#a6d189"
  code_background = "#292c3c"
  blockquote = "#a5adce"
  list_marker = "#ca9ee6"

  [ui.outline]
  background = "#292c3c"
  heading1 = "#8caaee"
  heading2 = "#a6d189"
  heading3 = "#e5c890"
  heading4 = "#ca9ee6"

  [ui.search]
  background = "#292c3c"
  border = "#8caaee"
  input = "#c6d0f5"
  match_highlight = "#ef9f76"
  match_current = "#e78284"
  match_count = "#a5adce"

  [ui.editor]
  heading1 = "#8caaee"
  heading2 = "#a6d189"
  heading3 = "#e5c890"
  heading4 = "#ca9ee6"
  heading5 = "#99d1db"
  heading6 = "#a5adce"
  code = "#a6d189"
  link = "#99d1db"
  blockquote = "#a5adce"
  list_marker = "#ca9ee6"
  bold = "#e5c890"
  italic = "#81c8be"
  ```

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Acceptance Criteria**:
  - File exists at `themes/catppuccin-frappe.toml`
  - Content matches exactly

- [ ] 3. Register Themes in Config

  **What to do**:
  - Modify `src/config.rs` in `Config::load_or_create`.
  - Add logic to write the two new themes if they don't exist.

  **References**:
  - `src/config.rs:169` - existing theme registration logic.

  **Instructions**:
  - Add:
    ```rust
    let latte_theme_path = themes_dir.join("catppuccin-latte.toml");
    if !latte_theme_path.exists() {
        let content = include_str!("../themes/catppuccin-latte.toml");
        let _ = fs::write(&latte_theme_path, content);
    }

    let frappe_theme_path = themes_dir.join("catppuccin-frappe.toml");
    if !frappe_theme_path.exists() {
        let content = include_str!("../themes/catppuccin-frappe.toml");
        let _ = fs::write(&frappe_theme_path, content);
    }
    ```

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Acceptance Criteria**:
  - `src/config.rs` includes the registration logic for both new themes.

---

## Success Criteria

### Verification Commands
```bash
cargo check
# Manual: Run app, Ctrl+P, switch to "Catppuccin Latte" and "Catppuccin Frappe"
```

### Final Checklist
- [ ] `themes/catppuccin-latte.toml` created
- [ ] `themes/catppuccin-frappe.toml` created
- [ ] `src/config.rs` updated
