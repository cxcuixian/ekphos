# Add Command Palette (Ctrl+p) with Theme Switching

## TL;DR

> **Quick Summary**: Implement a VS Code-style Command Palette triggered by `Ctrl+p`. The palette will allow searching and executing commands, starting with "Change Theme" functionality.
>
> **Deliverables**:
> - New `CommandPalette` functionality extending `SearchPicker`.
> - `Ctrl+p` keybinding in Normal mode.
> - Dynamic theme switching commands.
>
> **Estimated Effort**: Medium
> **Parallel Execution**: Sequential (due to state dependencies)
> **Critical Path**: State Updates → Logic Implementation → UI Integration → Event Binding

---

## Context

### Original Request
"Add `Command palette` function using keymap `Ctrl+p`, which can change theme types"

### Interview Summary
- **Approach**: Extend existing `SearchPicker` (used for `Ctrl+k`) to support a new `Commands` mode. This reuses the fuzzy search UI and logic.
- **Scope**:
  - Keybinding: `Ctrl+p` (Normal Mode).
  - Feature: Command list with fuzzy filtering.
  - Initial Command: Theme switching (lists available themes).
- **Theme Logic**: Will scan `themes/` directory and use internal defaults to populate "Theme: [Name]" commands.

### Metis/Self-Review
- **Gap Identified**: Persistence of theme changes.
- **Resolution**: Theme change will be runtime-only for this iteration (fast switching), matching the "preview" nature of command palettes. Persistence requires writing to `config.toml` which is a separate scope (Config management), but easy to add later.
- **Verification**: TUI interaction is hard to unit test. Will use `interactive_bash` for end-to-end verification.

---

## Work Objectives

### Core Objective
Enable users to execute commands via a searchable popup interface, starting with theme switching.

### Concrete Deliverables
- Modified `src/app/state.rs`: `SearchPickerMode::Commands` and command structures.
- Modified `src/ui/file_picker.rs`: Rendering logic for command results.
- Modified `src/event/handler.rs`: Input handling and keybinding.
- New `App` methods: `open_command_palette`, `execute_command`, `switch_theme`.

### Definition of Done
- [ ] Pressing `Ctrl+p` opens the palette with title "Command Palette".
- [ ] Typing "theme" filters the list to show available themes.
- [ ] Selecting a theme applies it immediately.
- [ ] `Esc` closes the palette.

### Must Have
- Reusing `SearchPicker` logic (DRY).
- "Theme: Dracula" and "Theme: Ekphos Dawn" (plus any in `themes/`) must be listed.

### Must NOT Have
- New UI widgets (use existing `Paragraph`, `List`, `SearchPicker` rendering).
- Dependencies on external fuzzy search crates (use existing matching logic).

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: Rust/Cargo (unit tests).
- **User wants tests**: Manual/E2E verification is more appropriate for TUI visual features.
- **Framework**: `interactive_bash` (tmux) for agent-driven E2E test.

### Automated Verification Procedures

**1. Verify UI and Keybinding**
```bash
# Agent runs via tmux:
1. Start app: cargo run
2. Wait for: "NORMAL" (indicates startup complete)
3. Send keys: "Ctrl+p"
4. Assert: Output contains "Command Palette" or "Search (Ctrl+P)"
5. Send keys: "Esc"
6. Assert: Output does NOT contain "Command Palette"
```

**2. Verify Theme Switching**
```bash
# Agent runs via tmux:
1. Start app: cargo run
2. Send keys: "Ctrl+p"
3. Send keys: "theme dracula"
4. Wait for: "Theme: Dracula" in output
5. Send keys: "Enter"
# Visual verification via screenshot if possible, or check log/state if available.
# Since we can't easily grep color codes, we assume success if no crash and palette closes.
```

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (State & Logic):
└── Task 1: Core State & Command Logic

Wave 2 (UI & Events):
└── Task 2: UI Implementation & Event Binding
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|------------|--------|
| 1 | None | 2 |
| 2 | 1 | None |

---

## TODOs

- [ ] 1. Implement Command Palette State & Logic

  **What to do**:
  - Modify `src/app/state.rs`:
    - Add `Commands` to `SearchPickerMode` enum.
    - Add `command_results: Vec<CommandResult>` to `SearchPickerState`.
    - Define `CommandResult` struct (display_name, action_id, score).
  - Implement `App::open_command_palette` in `src/app/state.rs` (or `mod.rs`):
    - Initialize picker in `Commands` mode.
    - Populate `command_results` with themes found in `themes/` and default themes.
  - Implement `App::execute_command` to handle action execution (e.g., `set_theme`).

  **References**:
  - `src/app/state.rs:SearchPickerMode` - Add variant here.
  - `src/app/state.rs:SearchPickerState` - Add results field here.
  - `src/app/state.rs:open_search_picker` - Use as template for `open_command_palette`.
  - `src/config.rs:Theme` - Refer for theme loading logic.

  **Acceptance Criteria**:
  - [ ] `cargo check` passes.
  - [ ] `SearchPickerMode::Commands` exists.
  - [ ] `App::open_command_palette` populates results.

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering` (Rust TUI State)
  - **Skills**: `git-master`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1

- [ ] 2. Implement UI Rendering & Event Handling

  **What to do**:
  - Modify `src/ui/file_picker.rs`:
    - Update `render_search_picker` to handle `SearchPickerMode::Commands`.
    - Change title to "Command Palette" when in Command mode.
    - Implement `render_command_results` (clone `render_file_results` but simplified).
  - Modify `src/event/handler.rs`:
    - In `handle_normal_mode`, bind `Ctrl+p` to `app.open_command_palette()`.
    - In `handle_search_picker_input`, add handling for `Commands` mode (filtering & execution).

  **References**:
  - `src/ui/file_picker.rs:render_search_picker` - Main rendering entry point.
  - `src/event/handler.rs:handle_normal_mode` - Add keybinding here.
  - `src/event/handler.rs:handle_search_picker_input` - Add input handling.

  **Acceptance Criteria**:
  - [ ] `Ctrl+p` opens the palette.
  - [ ] Palette title says "Command Palette" (or similar).
  - [ ] Typing filters the command list.
  - [ ] Enter executes the command (switches theme).

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering` (Rust TUI Implementation)
  - **Skills**: `git-master`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2

---

## Commit Strategy

| After Task | Message | Files |
|------------|---------|-------|
| 1 | `feat(state): add command palette state and theme logic` | `src/app/state.rs`, `src/app/mod.rs` |
| 2 | `feat(ui): implement command palette ui and ctrl+p binding` | `src/ui/file_picker.rs`, `src/event/handler.rs` |

---

## Success Criteria

### Final Checklist
- [ ] `Ctrl+p` opens Command Palette.
- [ ] "Theme: Dracula" is visible and selectable.
- [ ] Selecting a theme changes the UI colors immediately.
- [ ] No regressions in existing Search Picker (`Ctrl+k`).
