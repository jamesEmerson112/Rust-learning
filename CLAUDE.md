# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                      # Build everything
cargo run --bin c05_example      # Run a lesson example
cargo run --bin c05_exercise     # Run a lesson exercise
cargo test --test c05_tests      # Test a specific lesson
cargo test --tests               # Run all 33 lesson tests
```

## Architecture

A 33-lesson Rust curriculum. Each lesson is self-contained with three files:

- `src/bin/cXX_example.rs` — Complete reference implementation (read-only)
- `src/bin/cXX_exercise.rs` — Starter stubs with `TODO` comments (learner edits these)
- `tests/cXX_tests.rs` — Integration tests that validate the exercise

Tests import exercises as modules via `#[path = "../src/bin/cXX_exercise.rs"]` and test their `pub` functions. There is no `lib.rs`; lessons have no cross-dependencies.

**Lessons 32–33** introduce file-based modules in `src/lesson32/` and `src/lesson33/` (e.g., `exercise_counter.rs`, `exercise_gradebook.rs`). Their bin files and tests use `#[path]` attributes to import these modules.

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

### 2026-04-14
- [fix] Rewrote `src/bin/c17_example.rs` from dense `.map_err()?` chain to explicit `match` on `Result`. Original c17 introduced 3 new concepts at once (`.parse()`, `.map_err()`, `?`), violating one-concept-per-lesson rule — `?` belongs to c18, `map_err` to c19. Updated TODO comment in `src/bin/c17_exercise.rs` to hint at the match approach; test signature preserved.
- [verify] `cargo run --bin c17_example` prints `Ok(25)` and `Err("invalid age: abc")` as expected.
- [decision] Durable pedagogical rule: from c17 onward every lesson introduces exactly ONE new concept. Audit confirmed c18-c29 already comply; c30 is the intentional integration capstone.
- [decision] Do NOT restructure the 30-lesson layout to close the 4 "honest gaps" (custom error enums, explicit slices `&[T]`, struct lifetimes, `RefCell<T>`). Churn on save file + `progress.rs` + ranks outweighs marginal pedagogical gain. Gaps belong to Post-30 Track.
- [feat] Created `ROADMAP.md` at repo root: 30-row at-a-glance table, 9 phase groupings with one-sentence themes, ASCII prerequisite chain, explicit Post-30 Track naming the 4 gaps plus thiserror/anyhow, custom iterators, async, macros. Added one-line link to `ROADMAP.md` at top of `README.md`.
- [research] `.parse::<T>()` works on any `T: FromStr` (trait bound on a generic method — Rust's compile-time dispatch analog to C's `atoi`/`atof`/`strtol` family). `FromStr` formally lands in c23 (traits); generics/bounds in c24-c25.
- [research] Teacher's review of c17-c30: pedagogically sound. One-concept rule enforced, dependency ordering correct throughout. Optional enhancement considered: extend c26 to cover struct lifetimes as a second `'a` example — user deferred.
- [ref] Plan file `C:\Users\voan2\.claude\plans\spicy-foraging-frost.md` updated with teacher's review + ROADMAP plan.
- [feat] Completed c18 The `?` Operator — user implemented `parse_pair(a, b) -> Result<(i32, i32), ParseIntError>` using `?` on both `.parse::<i32>()` calls. All 4 tests pass (parse_pair_both_valid, parse_pair_negative_numbers, parse_pair_first_invalid, parse_pair_second_invalid).
- [feat] Completed c19 Error Chaining with `map_err` — studied the example (no exercise edit needed to confirm understanding).
- [feat] Completed c20 Closures — user implemented `apply(x, f: impl Fn(i32) -> i32) -> i32` returning `f(x)`. All 3 tests pass (apply_double, apply_add_constant, apply_captures_from_scope). Level 20/30.
- [research] `?` exits the enclosing *function*, not the program. The caller receives the `Err` and can handle it (match, propagate further, ignore). Only terminates the program if nobody handles it all the way up to `main`.
- [research] `.map_err()` transforms the error variant while leaving `Ok` untouched. Signature: `Result<T, E1>.map_err(E1 -> E2) -> Result<T, E2>`. Idiomatic pair: `.map_err(...)?` — convert error type, then early-return. Symmetric with `.map(...)` on the Ok side.
- [research] `.parse()` is generic over any `T: FromStr`, returns `Result<T, T::Err>`. Turbofish `::<T>` picks the target type. Rust's typed equivalent of C's `atoi`/`atof`/`strtol` family, but forces explicit error handling.
- [decision] Post-c30 track will emphasize practical error-handling projects (CLI tools, HTTP clients, REPL). Candidates surfaced: config-file validator, CSV summarizer, tiny HTTP client with reqwest+serde, simple calculator REPL, re-implementations of wc/head/tail. Noted in ROADMAP.md post-30 direction.
- [research] Teacher's "how useful are you now?" assessment after c20: reachable ~70-75% of LeetCode Easy; can write small CLI tools with proper error handling; cannot yet write idiomatic iterator chains (c21+), generic helpers (c23+), or borrowing structs (c26). c20 framed as the "quiet turning point" — closures + upcoming iterators (c21-22) shift the code from procedural to idiomatic Rust.
- [todo] Next: c21 Iterator Combinators I (map/filter/collect). Expected to be the "this language is elegant" moment.
- [ref] Unrelated cleanup flagged but not done: `src/bin/c17_exercise.rs:1` has `use std::fmt::format;` leftover (unused import warning).

### 2026-04-22
- [refactor] Expanded curriculum from 30 to 33 lessons to enforce one-concept-per-lesson rule. c21 (map+filter+collect) was overloaded with 3 new concepts; c22 (fold+sum+turbofish) also overloaded; c25 (trait bounds) used `.copied()` and `?` on Option which hadn't been taught.
- [refactor] Split old c21 → new c21 (Map+Collect) + c22 (Filter). Split old c22 → new c23 (Sum) + c24 (Fold). Added brand new c25 (Debug Format — `{:?}`). Simplified old c25 (trait bounds) → new c28 with `larger<T: Ord>` instead of complex iterator-based `largest`.
- [refactor] Renamed old c23→c26, c24→c27, c25→c28, c26→c29, c27→c30, c28→c31, c29→c32, c30→c33. Directories: lesson29/→lesson32/, lesson30/→lesson33/.
- [feat] progress.rs now dynamic: `NUM_LESSONS` and `MAX_XP` derived from `LESSONS.len()`. Stat bars scale to group size. Added "Debug Console" stat group, "Neon Sovereign" rank at L30, final rank at L33. 10 ranks, 16 stat groups, 33 abilities.
- [decision] No upper bound on lesson count. One concept per lesson is the hard rule; if a lesson smuggles extras, split it. Saved as durable feedback memory.
- [decision] c25 (Debug Format) placed as bridge between iterators and traits — motivates "what IS a trait?" before c26 formally teaches it.
