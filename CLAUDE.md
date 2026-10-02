# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

An 80-lesson Rust curriculum for one learner, a senior developer coming from C/C++, Python, and TypeScript. Explanations should lean on systems-level analogies (`&str` is roughly `const char*`, `Box<T>` is roughly `unique_ptr<T>`). The crate is edition 2024 and has no `lib.rs` on purpose.

README.md holds the per-lesson study plan (**Learn** / **Exercise** / **You're done when**). ROADMAP.md holds the at-a-glance table, the phase and chapter blurbs, and the prerequisite chain. Read those for what a lesson teaches and how its story is themed. This file covers the mechanics.

## Commands

```bash
cargo run --bin c57_example            # run a lesson's reference implementation
cargo run --bin c57_exercise           # run the learner's version
cargo check --bin c57_exercise         # fastest compile check of one file
cargo test --test c57_tests            # validate one lesson
cargo test --test c57_tests total      # run only tests whose name contains "total"
cargo test --tests                     # all 80 suites; unfinished lessons fail by design
cargo check --bins                     # confirm every example and exercise compiles

cargo run --bin progress               # incremental scan, updates the save, prints the character sheet
cargo run --bin progress -- --rescan   # re-test all 80 lessons (`-- --reset` deletes the save)
cargo run --bin dashboard              # write dashboard.html and open it (`-- --no-open` to skip opening)
```

Flags for `progress` and `dashboard` need the `--` separator, as shown.

## One broken file blocks every lesson's tests

Cargo builds every binary target in the package before it runs any integration test, and this package has 163 of them (80 examples, 80 exercises, `progress`, `dashboard`, and `src/main.rs`). If any single example or exercise fails to compile, `cargo test --test cXX_tests` fails for every lesson, including lessons unrelated to the broken file.

`progress` runs exactly that command once per lesson with output silenced. A scan in that state therefore marks every lesson as failed and appends `regression` events to the save's history, which is never cleared. Run `cargo check --bins` and get it clean before running `progress`.

This is also why learner-form exercises must always compile. Stubs use `let _ = (...);` placeholders with a dummy return value, and imports the solution will need are marked `#[allow(unused_imports)]`.

## Layout and wiring

Each lesson is a triple. `src/bin/cXX_example.rs` is the complete reference and the canonical answer. `src/bin/cXX_exercise.rs` is the learner's file, holding `TODO` stubs or a planted bug. `tests/cXX_tests.rs` pulls the exercise in as a module with `#[path = "../src/bin/cXX_exercise.rs"] mod cXX_exercise;` and tests its `pub` items, so exercise functions and types must be `pub`. Every exercise also keeps a `main()` so `cargo run` works on it.

All code sharing goes through `#[path]`, never a library crate:

- c32 and c33 load their modules from `src/lesson32/` and `src/lesson33/`.
- `c74_exercise.rs` imports the learner's own `c71_exercise.rs`. This is the only cross-lesson dependency, and c74's tests tell the learner to finish c71 first. Only c71 is imported, because importing c73 as well would create two distinct `Stock` types.
- `progress` and `dashboard` both include `src/tracker.rs`. Only `dashboard` includes `src/knowledge_tree.rs`.

The async lessons (c50–c52, c80) use `#[tokio::test]`. The sled lessons (c69–c74) test against `sled::Config::new().temporary(true)`, while their examples write `*_sled_db` directories into the working directory, which are gitignored.

`benchmarks/`, `python_exercises/`, `c02_example_backup.rs`, and `src/main.rs` are side experiments outside the lesson pipeline. Nothing reads them.

## Tracker and dashboard

`src/tracker.rs` holds all lesson metadata as fixed-size const arrays: `LESSONS`, `ABILITIES`, `STAT_GROUPS`, `RANKS`, `CHAPTERS`, `BOSSES`, `CLASSES`, `BUG_LESSONS`, and `HOMEWORK_GAPS`. The lengths are part of each array's type, so adding a lesson means updating the counts together. `NUM_LESSONS` and `MAX_XP` derive from `LESSONS.len()`. `BUG_LESSONS` is the authoritative list of bug-hunt lessons. `HOMEWORK_GAPS` lists lessons the learner deliberately left open, and the dashboard skips them when it picks the next lesson.

`progress` owns every write to `.rustacean_save.json` (gitignored, schema v3). `dashboard` only reads it, and writes a self-contained `dashboard.html` with inline SVG, no JavaScript, and no network access. An incremental scan starts one lesson before the first unpassed lesson, so it re-verifies the last pass. `--rescan` re-tests everything but carries `first_passed_at`, `attempts`, and `backfilled` forward. The `history` event log is append-only. Heatmap days are UTC. Older saves upgrade through `migrate()` in `src/tracker.rs`, one version step at a time. Entries migrated from a v1 save are flagged `backfilled` and are excluded from streak and pace math. The v2 to v3 step renames the four cyberpunk character classes to their hospital equivalents.

`src/knowledge_tree.rs` is a 223-node map of the Rust ecosystem shown only by the dashboard. Each node's `lessons: &[u32]` ties it to curriculum lessons, and most nodes are deliberately empty to show Rust the curriculum doesn't cover. Node text contains HTML-special characters such as `<` and `>`, so the dashboard must pass it through `esc()`.

Story-specific strings live in the `tracker.rs` arrays and in the trophy list in `dashboard.rs`. A re-theme has to touch both, in addition to the lesson files, README.md, and ROADMAP.md.

## Lesson formats and authoring rules

There are three exercise formats. Plain lessons ship `TODO` stubs. ⚡ warmups (c55, c59, c62, c65) are small DSA problems solvable with c01–c54 tools only. ★ bug hunts (the lessons in `BUG_LESSONS`) ship code that compiles but fails its tests.

These rules hold for any new or rewritten lesson:

- Each lesson introduces exactly one new concept. Warmups and bug hunts introduce none. There is no cap on lesson count, so a lesson that needs two concepts gets split.
- Author solved first. Write the exercise solved, run its suite green, convert it to learner form by stubbing it or planting the bug, then confirm the suite fails deterministically.
- A bug lesson always compiles. It fails by a wrong value, an `Err`, or an expected panic, and never by a hang or anything nondeterministic. A `// BUG:` comment at the top states the symptom and never the fix. The example file is the corrected reference.
- Example style differs by era. c01–c30 are lean, with one function, a short `main`, and no explainer comments. Later examples open with a header comment that explains the concept and includes a "Coming from C" analogy. Match the neighboring lessons.
- When adding or renumbering a lesson, update all of these together: the arrays in `src/tracker.rs`, any affected `lessons:` tags in `src/knowledge_tree.rs`, the ROADMAP.md table row, phase blurb, and prerequisite chain, and the README.md Study Plan entry (`### Lesson N — Title` with **Learn** / **Exercise** / **You're done when**).

The learner's real progress is in `.rustacean_save.json`. Read it, or run `progress`, rather than trusting a status written in any doc. Earlier versions of this file kept a session-by-session history, which is preserved in `git log -p -- CLAUDE.md`.
