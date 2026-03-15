# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                      # Build everything
cargo run --bin c05_example      # Run a lesson example
cargo run --bin c05_exercise     # Run a lesson exercise
cargo test --test c05_tests      # Test a specific lesson
cargo test --tests               # Run all 20 lesson tests
```

## Architecture

A 20-lesson Rust curriculum. Each lesson is self-contained with three files:

- `src/bin/cXX_example.rs` — Complete reference implementation (read-only)
- `src/bin/cXX_exercise.rs` — Starter stubs with `TODO` comments (learner edits these)
- `tests/cXX_tests.rs` — Integration tests that validate the exercise

Tests import exercises as modules via `#[path = "../src/bin/cXX_exercise.rs"]` and test their `pub` functions. There is no `lib.rs`; lessons have no cross-dependencies.

**Lessons 19–20** introduce file-based modules in `src/lesson19/` and `src/lesson20/` (e.g., `exercise_counter.rs`, `exercise_gradebook.rs`). Their bin files and tests use `#[path]` attributes to import these modules.

## Progress Tracker

`cargo run --bin progress` — RPG-style character progression that scans test results.
- Save file: `.rustacean_save.json` (gitignored)
- Flags: `--rescan` (re-test all), `--reset` (delete save), `--help`
- Dependencies: `serde`, `serde_json`

## Conventions

- Exercise functions must be `pub` so tests can import them
- Each exercise file includes a `main()` for `cargo run` and `pub fn`s for `cargo test`
- Examples are complete — exercises mirror the same API but with stubs
- The learner is a senior dev (C/C++, Python, JS/TS background) learning Rust; use systems-level analogies (e.g., `&str` ≈ `const char*`)
- Edition is 2024
