# Implement Vim Case Transformation

## TL;DR

> **Quick Summary**: Add support for converting selected text to uppercase (`U`) or lowercase (`u`) in Vim Visual Mode (Standard, Line, and Block).
> 
> **Deliverables**:
> - Logic in `Editor` to transform text in selections.
> - Keybindings in `handle_vim_visual_mode`.
> 
> **Estimated Effort**: Short
> **Parallel Execution**: Sequential
> **Critical Path**: Editor Logic -> Event Handler

---

## Context

### Original Request
"In vim visual mode, add u=Marked area converted to lower case,U=Convert marked area to uppercase function"

### Analysis
- **Standard Vim Behavior**:
  - `u`: lowercase selection.
  - `U`: uppercase selection.
  - Action exits visual mode.
  - Works on all visual modes (char, line, block).

---

## Work Objectives

### Core Objective
Enable case switching on selected text.

### Concrete Deliverables
- `src/editor/mod.rs`: `transform_selection_case` method.
- `src/event/handler.rs`: Key bindings.

### Definition of Done
- [ ] Selecting text and pressing `u` makes it lowercase.
- [ ] Selecting text and pressing `U` makes it uppercase.
- [ ] Visual mode exits after action.
- [ ] Works for Visual Line and Visual Block modes too.

---

## Verification Strategy

### Automated Verification
None (Interaction based).

### Manual Verification
1.  **Run**: `cargo run`
2.  **Test Standard Visual**: `v`, select text, `U` -> Uppercase?
3.  **Test Visual Line**: `V`, select lines, `u` -> Lowercase?
4.  **Test Visual Block**: `Ctrl+v`, select block, `U` -> Uppercase?

---

## Execution Strategy

### Parallel Execution Waves
Sequential.

---

## TODOs

- [ ] 1. Implement Transformation Logic

  **What to do**:
  - Modify `src/editor/mod.rs`.
  - Add `pub fn transform_selection_case(&mut self, to_upper: bool)`.
  - Handle `visual_line_selection`, `visual_block_selection`, and standard selection (from `cursor.selection_range()`).
  - Use Rust's `to_uppercase()` / `to_lowercase()`.
  - Record history for undo.

  **Recommended Agent Profile**:
  - **Category**: `ultrabrain`
  - **Skills**: `["git-master"]`

- [ ] 2. Bind Keys

  **What to do**:
  - Modify `src/event/handler.rs`.
  - In `handle_vim_visual_mode`:
    - Match `KeyCode::Char('u')`: Call `transform_selection_case(false)`, then exit visual mode.
    - Match `KeyCode::Char('U')`: Call `transform_selection_case(true)`, then exit visual mode.
  - Ensure cursor style updates and mode resets to Normal.

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `["git-master"]`

---

## Success Criteria

### Final Checklist
- [ ] `cargo check` passes.
- [ ] Code compiles.
