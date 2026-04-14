# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                      # Build everything
cargo run --bin c05_example      # Run a lesson example
cargo run --bin c05_exercise     # Run a lesson exercise
cargo test --test c05_tests      # Test a specific lesson
cargo test --tests               # Run all 30 lesson tests
```

## Architecture

A 30-lesson Rust curriculum. Each lesson is self-contained with three files:

- `src/bin/cXX_example.rs` — Complete reference implementation (read-only)
- `src/bin/cXX_exercise.rs` — Starter stubs with `TODO` comments (learner edits these)
- `tests/cXX_tests.rs` — Integration tests that validate the exercise

Tests import exercises as modules via `#[path = "../src/bin/cXX_exercise.rs"]` and test their `pub` functions. There is no `lib.rs`; lessons have no cross-dependencies.

**Lessons 29–30** introduce file-based modules in `src/lesson29/` and `src/lesson30/` (e.g., `exercise_counter.rs`, `exercise_gradebook.rs`). Their bin files and tests use `#[path]` attributes to import these modules.

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

## Context History

### 2026-04-13
- [progress] Advanced from c07 through c12; all tests c01-c12 passing. Currently Level 12/20, "ICE Breaker" rank, 1200/2000 XP (Rustacean handle: Chrome Surgeon).
- [feat] Completed c07 Ownership/Borrowing (`shout()`), c08 String Slices, c09 Structs, c10 Methods/impl, c11 Enums, c12 Option<T>.
- [fix] c11_example.rs — removed unused `std::fmt::format` import and added missing "Result: " prefix in format strings; also fixed duplicate function name + string indexing issue.
- [fix] Renamed `src/bin/c02_example copy.rs` to `c02_example_backup.rs` to resolve cargo crate-name error (spaces invalid in bin target names).
- [research] Discussed: 4 String constructors (`String::from`, `.to_string`, `.to_owned`, `.into`) and their differing traits; `iter()` vs `iter_mut()` vs `into_iter()`; range `i..n` vs `i..=n`; no function overloading / no classes; strings not indexable (use `.starts_with()`); `.next()` advances iterator cursor; HashMap `entry().or_insert(0)` counting idiom; `usize` ≈ `size_t`.
- [todo] Working on c13 HashMap Basics exercise — user hit bug using `.keys()` instead of `.entry(c)` for the counting pattern.
- [feat] Expanded curriculum from 20 to 30 lessons using "split dense lessons + add new topics" approach. Capstone moved to c30; `progress.rs` updated accordingly.
- [refactor] Bulk-renamed via `git mv`: old c15->c17, c16->c19, c17->c23, c18->c24, c19->c29, c20->c30; also `src/lesson19/`->`src/lesson29/` and `src/lesson20/`->`src/lesson30/`.
- [refactor] Split dense lessons: old c14 -> new c14/c15/c16; old c16 -> c18/c19; old c18 -> c24/c25.
- [feat] Authored 6 new lessons: c20 Closures, c21 Iter Combinators I (map/filter), c22 Iter Combinators II (fold/sum), c26 Lifetimes Intro, c27 Box<T>, c28 Rc<T>/Arc<T>.
- [feat] New curriculum c14-c30: c14 HashMap Entry API, c15 String Normalization, c16 Word Count, c17 Result Basics, c18 `?` Operator, c19 Error Chaining (map_err), c20 Closures, c21 Iter I, c22 Iter II, c23 Traits, c24 Generics, c25 Trait Bounds, c26 Lifetimes, c27 Box<T>, c28 Rc<T>/Arc<T>, c29 Modules, c30 Capstone Gradebook.
- [feat] progress.rs: LESSONS 20->30, STAT_GROUPS 10->15 (added String Forge, Error Channel, Iterator Matrix, Bound Compiler, Pointer Grid), ABILITIES 20->30, RANKS 7->9 (added Neon Assassin @L21, Silicon Shaman @L25; Zero-Day Sovereign moved to L30), max XP bar 2000->3000.
- [decision] User wants lean c10-c13 style (one function, 1-3 line main, no explainer comments) for all c1-c30 examples while still learning; richer/denser style reserved for post-c30 material. Slimmed c14, c15, c16, c18, c20, c21, c22, c24, c25, c26, c27, c28 examples accordingly.
- [verify] `cargo build --all` clean (one pre-existing c10 warning); all 30 test files compile; `cargo run --bin progress` displays 30-lesson curriculum. User advanced Level 12 -> 13 (c13 complete); c14 shown as NEXT.
- [ref] Plan file: `C:\Users\voan2\.claude\plans\spicy-foraging-frost.md`.
- [feat] Completed c14 (HashMap Entry API — `count_numbers`) and c15 (String Normalization — `normalize_word`); around Level 14-15/30.
- [todo] Working on c16 Word Count (HashMap + normalization combo). Two bugs hit: (1) inserted raw `&str` into `HashMap<String, usize>` — fix with `.to_string()`; (2) `normalise()` returning `String` but `.trim_matches()` returns `&str` — fix with `.to_lowercase()` or `.to_string()` to convert.
- [research] `usize` vs `i32` (usize = 8 bytes on 64-bit, ≈ C `size_t`, used for counts/indices to signal intent and avoid casts).
- [research] DSA "minimum value" pattern map: linear scan, sliding window, monotonic deque, binary search, heap, DP — not always DP.
- [research] Estimated current Rust knowledge (c01-c13) covers ~60% of LeetCode "Easy" assuming algorithm knowledge.
- [research] `.to_lowercase()` vs `.to_ascii_lowercase()` vs `.to_uppercase()` — Unicode-aware vs ASCII-only.
- [research] `&str` vs `String`: viewing methods return `&str` (no allocation), transforming methods return `String` (new data).
- [decision] User commits to Rust as sole learning track; explicitly chose Rust over Python/DSA/LeetCode grinding despite immediate-ROI argument. Reason: Python carries negative emotional baggage from two recent interview failures; Rust = fresh canvas + momentum. Sustainable learning > optimal learning. Saved as durable feedback memory `feedback_rust_only_learning_track.md`.
