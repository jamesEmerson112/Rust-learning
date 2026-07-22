# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                      # Build everything
cargo run --bin c05_example      # Run a lesson example
cargo run --bin c05_exercise     # Run a lesson exercise
cargo test --test c05_tests      # Test a specific lesson
cargo test --tests               # Run all 80 lesson tests
```

## Architecture

An 80-lesson Rust curriculum. Each lesson is self-contained with three files:

- `src/bin/cXX_example.rs` — Complete reference implementation (read-only)
- `src/bin/cXX_exercise.rs` — Starter stubs with `TODO` comments (learner edits these)
- `tests/cXX_tests.rs` — Integration tests that validate the exercise

Tests import exercises as modules via `#[path = "../src/bin/cXX_exercise.rs"]` and test their `pub` functions. There is no `lib.rs`; lessons have no cross-dependencies — with ONE deliberate exception: `c74_exercise.rs` imports the learner's own `c71_exercise.rs` via `#[path]` (the capstone assembles their codec).

**Lessons 32-33** introduce file-based modules in `src/lesson32/` and `src/lesson33/` (e.g., `exercise_client_counter.rs`, `exercise_service_log.rs`). Their bin files and tests use `#[path]` attributes to import these modules.

**Lessons 34-54** are nail salon themed (the user runs a salon and is building a scheduler). Vietnamese technician names throughout (Mai, Linh, Trang). Topics: error handling II (c34-c38), slices/lifetimes II (c39-c41), interior mutability (c42-c44), custom iterators (c45-c46), file I/O (c47-c49), async/tokio (c50-c52), CLI with clap (c53-c54).

**Lessons 55-74** are THE VAULT RUN — a five-chapter cyberpunk heist arc (rewritten 2026-07 because the learner found the original flat and stopped at c55). Same concepts as before, one story: the learner is netrunner "Chrome Surgeon" breaching the Aegis-9 vault; Mai/Linh/Trang carry over as the crew. Chapters: LOADOUT (c55-c58, Box/trait objects/Deref), GHOST PROTOCOL (c59-c61, Drop/Weak), INSIDE THE ICE (c62-c64, Cell/RefCell), THE CREW (c65-c68, Arc/Mutex/RwLock over `std::thread`), THE VAULT (c69-c74, sled + serde, capstone imports the learner's own c71). Three exercise formats: plain stubs, ⚡ warmups (c55/c59/c62/c65, c01-c54 tools only), and ★ BUG HUNT lessons (c58/c61/c64/c67) that ship compiling-but-wrong code with a `// BUG:` symptom comment — tests fail deterministically, never hang. Examples keep the "Coming from C / ThreadX" header-note style.

**Lessons 75-80** are the Bug Hunt block — salon-themed debugging side jobs (the learner prefers debugging to writing from scratch). Every exercise compiles but fails its tests via one classic bug archetype per lesson: HashMap insert-clobber (c75), swallowed parse error (c76), inverted filter (c77), slice off-by-one (c78), RefCell double borrow (c79), half-drained tokio mpsc (c80). No new concepts — all c01-c54 material; the example file is the corrected reference.

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

## Current Status (as of 2026-07-22)

- **Frontier: c55** — The Vault Run, Chapter 1 (LOADOUT). Resume with `cargo run --bin c55_example`, then edit `src/bin/c55_exercise.rs`, test with `cargo test --test c55_tests`.
- **Passed** (verified by the 2026-07-21 full rescan): all of c01-c54 EXCEPT open homework **c16, c19, c35, c37-c38, c40, c48-c49, c53**. The Vault Run does not depend on these.
- **Untouched: c55-c80** — all learner stubs / planted bugs.
- `.rustacean_save.json` mirrors this state. Update this section when progress meaningfully changes.

## Authoring New Lessons

Standing process rules (established 2026-06-08, reused 2026-07-21):

- **One new concept per lesson** — the hard rule. If a lesson smuggles extras, split it. Warmups (⚡) and Bug Hunt (★) lessons introduce ZERO new concepts.
- **Solved-first verification flow:** author exercises SOLVED with their tests → run the suites green (proves wiring and solvability) → convert to learner form (stubs: `let _ = ...;` placeholders + `#[allow(unused_imports)]`; bug lessons: plant the defect) → confirm every suite FAILS deterministically. Examples ship complete.
- **Bug-lesson rules:** the code always compiles; tests fail deterministically (wrong value / `Err` / expected panic) — never a hang, no nondeterminism; a `// BUG:` comment states the SYMPTOM, never the fix; the example file is the corrected reference.
- **When adding lessons**, extend in lockstep: `src/bin/progress.rs` (LESSONS, ABILITIES, STAT_GROUPS, RANKS — the counts in the array types must match), ROADMAP.md (table row + phase blurb + prereq chain), README.md Study Plan entry (`### Lesson N — Title` + **Learn/Exercise/You're done when**).

## Context History

Older sessions are condensed; full detail is in this file's git history (`git log -p CLAUDE.md`).

### 2026-04 (summary)
- [refactor] Curriculum grew 20→30→33: dense lessons split so each teaches one concept (map/collect, filter, sum, fold became separate lessons; Debug Format added as the iterator→traits bridge). Renames rippled through progress.rs and lesson directories.
- [decision] One-concept-per-lesson locked in from c17 onward (c17 itself rewritten from a `.map_err()?` chain to an explicit `match` for this reason). No upper bound on lesson count (memory: `feedback_no_lesson_cap`).
- [decision] Lean example style for c01-c30 (one function, 1-3 line main, no explainer comments); richer style reserved for later material.
- [decision] User committed to Rust as the sole learning track (memory: `feedback_rust_only_learning_track`).
- [feat] ROADMAP.md created (at-a-glance table, phase groupings, prereq chain, post-track). progress.rs made dynamic: `NUM_LESSONS`/`MAX_XP` derived from `LESSONS.len()`.
- [progress] Learner advanced roughly c07 → c20 across April. Research topics that month (String constructors, iterator families, `usize` ≈ `size_t`, `parse`/`FromStr`, `map_err`, `&str` vs `String`) are in git history.

### 2026-06-08 (summary)
- [feat] Expanded 54→68: Smart Pointers Deep Dive (Box for recursion/trait objects/Deref, Drop/Weak, full Cell/RefCell API, Arc/Mutex/RwLock — first `std::thread` use) plus 4 light DSA warmups, one per cluster, as an on-ramp to the difficulty jump. All std, no new deps.
- [feat] progress.rs extended (ranks Heap Warden/Memory Reaper/Concurrency Daemon; endgame moved to L68). Solved-first verification flow established — now codified under "Authoring New Lessons" above.
- (This block's exercises were superseded by THE VAULT RUN rewrite on 2026-07-21.)

### 2026-07-21
- [refactor] Rewrote c55-c74 as "THE VAULT RUN" — a five-chapter cyberpunk heist arc (LOADOUT, GHOST PROTOCOL, INSIDE THE ICE, THE CREW, THE VAULT). User had stopped at c55 (bored/discouraged by flat isolated stubs); test run verified c50-c52+c54 passed from the Jul 9 session and everything c55+ was untouched. Concepts per lesson unchanged; theme, exercise formats, and richness rewritten. Salon crew (Mai/Linh/Trang) carries over as the heist crew; learner plays "Chrome Surgeon" (their save-file class).
- [decision] User picked (via AskUserQuestion): cyberpunk netrunner theme matching the RPG tracker; a MIX of exercise formats (fix-the-bug + build-on-own-code + richer stubs); scope c55-c74 only. Mid-turn addition: salon-themed EXTRA exercises as debugging problems — "i prefer debugging" (saved as durable memory feedback_prefers_debugging_exercises).
- [feat] Four ★ BUG HUNT lessons in the arc: c58 (Deref serves factory_default instead of firmware), c61 (strong Rc cycle keeps the trace alive), c64 (RefCell write-during-sweep panic → try_borrow_mut), c67 (deposits increment a copied value, not the shared Mutex — compiler warning is the clue).
- [feat] c74 finale imports the learner's OWN c71_exercise.rs via `#[path]` (the one deliberate cross-lesson dependency) — capstone assembles their Intel codec; tests hint "finish c71 first" if it's still a stub. Only ONE import on purpose: importing both c71 and c73 would create two distinct Intel types.
- [feat] c75-c80 "Bug Hunt" block (authored by Opus subagent, verified green-solved/red-bugged both directions): salon debugging side jobs — HashMap insert clobber (c75), swallowed parse error (c76), inverted filter (c77), slice off-by-one (c78), RefCell double borrow (c79), half-drained tokio mpsc (c80).
- [feat] progress.rs: LESSONS 74→80, STAT_GROUPS 29→30 ("Embedded Store"→"Datavault", added "Bug Hunt"), ABILITIES 74→80, RANKS 18→19 (rank 74 "Salon Sovereign"→"Vault Sovereign"; new final rank "Zero-Day Sovereign" @L80). Endgame banner rethemed.
- [verify] Full flow: solved-reference pass = 20/20 suites green (55 tests); learner-form pass = 20/20 suites fail deterministically; cargo build clean; all 20 examples run with story output; `progress --rescan` over 80 lessons updated the stale save to ground truth.
- [docs] ROADMAP.md (80 rows, ★ tag, chapter phase blocks, new prereq chain, Post-80 Track), README.md (Study Plan c55-c80 with chapter lead-ins, File layout c01…c80 drift fixed), CLAUDE.md architecture notes.
- [ref] Solved references for c55-c74 lived in the session scratchpad only (not committed) — the examples are the canonical answers.
