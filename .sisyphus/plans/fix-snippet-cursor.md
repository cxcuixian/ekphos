# Fix Snippet Cursor Positioning

## TL;DR

> **Quick Summary**: Fix a bug where snippet tabstops (cursor positions) were calculated incorrectly due to mixing byte counts and character counts.
> 
> **Deliverables**:
> - Fix in `src/editor/mod.rs` to use character counting.
> 
> **Estimated Effort**: Trivial
> **Parallel Execution**: Sequential
> **Critical Path**: Update `select_tabstop_range`

---

## Context

### Original Request
"prefix": "bal" ... "body": ... when I use `bal`, the cursor should move to `$0` position, but now it moves another, fix the bug

### Analysis
- **Issue**: `select_tabstop_range` used `line.len()` (bytes) instead of `line.chars().count()` (chars).
- **Impact**: Incorrect cursor positioning if multibyte characters exist (or even just logic inconsistency).

---

## Work Objectives

### Core Objective
Ensure cursor lands on `$0` (and other tabstops) correctly.

### Concrete Deliverables
- `src/editor/mod.rs`: Fix `select_tabstop_range`.

### Definition of Done
- [ ] Code uses `chars().count()`.

---

## Verification Strategy

### Automated Verification
Code check.

### Manual Verification
1.  **Run**: `cargo run`
2.  **Test**: Type `bal` and verify cursor position.

---

## Execution Strategy

### Parallel Execution Waves
Sequential.

---

## TODOs

- [ ] 1. Fix Byte/Char Mismatch

  **What to do**:
  - Modify `src/editor/mod.rs`.
  - In `select_tabstop_range`:
    - Replace `let line_len = line.len();` with `let line_len = line.chars().count();`.

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `["git-master"]`

---

## Success Criteria

### Final Checklist
- [ ] `cargo check` passes.
