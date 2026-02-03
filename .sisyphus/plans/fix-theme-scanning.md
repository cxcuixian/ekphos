# Fix Theme Directory Scanning

## TL;DR

> **Quick Summary**: Update `get_available_themes` to use the canonical `Config::themes_dir()` path instead of guessing with `dirs::config_dir()`.
> 
> **Deliverables**:
> - Update `src/app/state.rs`
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: Sequential

---

## Context

### Original Request
Ensure `Ctrl+p` scans `/Users/aires/.config/ekphos/themes`.

### Diagnosis
The current implementation in `src/app/state.rs` uses `dirs::config_dir()`, which on macOS resolves to `~/Library/Application Support/`. The configuration system (`src/config.rs`) explicitly uses `~/.config/`. This mismatch causes themes saved in `~/.config/ekphos/themes` to be ignored by the Command Palette.

---

## Work Objectives

### Core Objective
Make `Ctrl+p` list all themes from `~/.config/ekphos/themes`.

### Concrete Deliverables
- Modified `src/app/state.rs`

### Definition of Done
- [ ] `get_available_themes` uses `crate::config::Config::themes_dir()`
- [ ] Manual test confirms themes appear

---

## Verification Strategy

### Manual Verification
1. **Build**: `cargo run`
2. **Check**: `Ctrl+p` should list "Catppuccin Latte" and "Catppuccin Frappe" (which were saved to `~/.config/ekphos/themes` in the previous task).

---

## TODOs

- [ ] 1. Fix `get_available_themes` implementation

  **What to do**:
  - Replace the `dirs::config_dir()` logic with `crate::config::Config::themes_dir()`.

  **Reference**:
  - `src/app/state.rs:6139`

  **Code Change**:
  ```rust
  // OLD
  if let Some(config_dir) = dirs::config_dir() {
      let themes_dir = config_dir.join("ekphos").join("themes");
      // ...
  }

  // NEW
  let themes_dir = crate::config::Config::themes_dir();
  if let Ok(entries) = fs::read_dir(&themes_dir) {
      // ...
  }
  ```

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Acceptance Criteria**:
  - Code uses `Config::themes_dir()`
  - Logic flow handles directory reading safely
