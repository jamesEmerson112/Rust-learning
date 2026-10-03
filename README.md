# Rust Learning

See [ROADMAP.md](ROADMAP.md) for a scannable overview of all 80 lessons.

This repo contains small Rust lessons, each with:
- an `example` binary (complete reference)
- an `exercise` binary (starter stubs with `TODO`s)
- an integration test file for validation

## File layout
- `src/bin/c01_example.rs` ... `src/bin/c80_example.rs`
- `src/bin/c01_exercise.rs` ... `src/bin/c80_exercise.rs`
- `tests/c01_tests.rs` ... `tests/c80_tests.rs`

## Lesson flow (~15 minutes each)
1. Read and run the example.
2. Open the matching exercise file.
3. Fill in `TODO` sections to match behavior.
4. Run the matching test file until green.

## Commands
Run a lesson example:
```bash
cargo run --bin c05_example
```

Run a lesson exercise:
```bash
cargo run --bin c05_exercise
```

Run validation for one lesson:
```bash
cargo test --test c05_tests
```

Run all lesson tests:
```bash
cargo test --tests
```

## Track your progress

Two views over the same save file (`.rustacean_save.json`, gitignored).

Terminal character sheet — scans your test results, levels you up, unlocks abilities:
```bash
cargo run --bin progress            # incremental scan
cargo run --bin progress -- --rescan  # re-test all 80
```

Gamified HTML dashboard — XP ring, chapter radar, activity heatmap, chapter map, boss HP bars, trophy case, and pace stats:
```bash
cargo run --bin dashboard           # writes dashboard.html and opens it
cargo run --bin dashboard -- --no-open
```

`progress` owns all writes; `dashboard` only reads. Run `progress` first, then `dashboard`.

## Study Plan

Work through one lesson at a time. Read the example, then complete the exercise. Don't move on until the exercise compiles and runs correctly.

---

### Lesson 1 — Hello Variables
**Learn:** `let`, `mut`, basic types (`i32`, `u8`, `bool`)
**Exercise:** Write a function that doubles a number.
**You're done when:** You can declare variables with explicit types and return values from functions.

---

### Lesson 2 — Strings and Formatting
**Learn:** `&str` vs `String`, `format!`, `println!`
**Exercise:** Write a function that returns a formatted greeting string.
**You're done when:** You understand why removing a semicolon changes what a function returns.

---

### Lesson 3 — Arrays and Iteration
**Learn:** Fixed-size arrays `[T; N]`, `.iter()`, `.sum()`
**Exercise:** Sum all elements of a four-element array.
**You're done when:** You can iterate over an array and use iterator methods.

---

### Lesson 4 — Tuples and Type Casting
**Learn:** Tuple returns, destructuring, `as` casting
**Exercise:** Return the min and max of a five-element array as a tuple.
**You're done when:** You can write a function that returns multiple values and cast between numeric types.

---

### Lesson 5 — If/Else and For Loops
**Learn:** `if`/`else`, `for` ranges, `Vec::new()`, `.push()`
**Exercise:** Collect all even numbers from 1 to n into a `Vec`.
**You're done when:** You can use a loop with a condition to build a vector.

---

### Lesson 6 — Match and String Building
**Learn:** `match`, `.to_string()`, `.join()`
**Exercise:** Implement FizzBuzz returning a `Vec<String>`.
**You're done when:** You can use `match` with multiple patterns and build a vector in a loop.

---

### Lesson 7 — Ownership and Borrowing
**Learn:** Ownership rules, `String::from()`, borrowing with `&`
**Exercise:** Write a function that uppercases a borrowed string and appends "!".
**You're done when:** You can explain why passing `&str` instead of `String` avoids transferring ownership.

---

### Lesson 8 — String Slices and Methods
**Learn:** `.split_whitespace()`, `.next()`, `Option` with `.map_or()`
**Exercise:** Find the length of the first word in a string slice.
**You're done when:** You can chain iterator methods and handle `Option` values.

---

### Lesson 9 — Structs
**Learn:** `struct` definition, instances, field access, `#[derive(Debug)]`
**Exercise:** Create a `Point` struct and compute its distance from the origin.
**You're done when:** You can define a struct and pass references to it.

---

### Lesson 10 — Methods and impl Blocks
**Learn:** `impl`, `&self`, methods taking references
**Exercise:** Add `area()` and `can_hold()` methods to a `Rectangle` struct.
**You're done when:** You can add behaviour to a struct and call methods on it.

---

### Lesson 11 — Enums
**Learn:** `enum` definition, variants with data, matching on enums
**Exercise:** Match on command variants and return result strings.
**You're done when:** You can define an enum with data and match on every variant.

---

### Lesson 12 — Option\<T\>
**Learn:** `Option<T>`, `Some`/`None`, `.unwrap_or()`, `if let`
**Exercise:** Safely retrieve an element from a slice by index.
**You're done when:** You can return `Some` or `None` and handle both cases.

---

### Lesson 13 — HashMap Basics
**Learn:** `HashMap`, `.insert()`, `.get()`, iteration
**Exercise:** Count the frequency of each character in a string.
**You're done when:** You can build and query a `HashMap`.

---

### Lesson 14 — HashMap Entry API
**Learn:** `.entry()`, `.or_insert()`, `.or_default()`, `.and_modify()`
**Exercise:** Count occurrences of each integer in a slice using the entry API.
**You're done when:** You can use the entry API without looking up the syntax.

---

### Lesson 15 — String Normalization
**Learn:** `.to_lowercase()`, `.trim_matches()`, char predicates like `.is_alphanumeric()`
**Exercise:** Clean a single word — lowercase it and strip surrounding punctuation.
**You're done when:** You can write a normalization function that handles messy input.

---

### Lesson 16 — Word Count (HashMap + Strings)
**Learn:** Combining the entry API with string normalization on real input
**Exercise:** Count word frequencies in a sentence, normalising case and punctuation.
**You're done when:** You can compose a multi-step text-processing pipeline.

---

### Lesson 17 — Result Basics
**Learn:** `Result<T, E>`, `Ok`/`Err`, `.parse::<T>()`, match on Result
**Exercise:** Parse a string as a `u8` age, returning `Err` for invalid input.
**You're done when:** You can return meaningful error messages from a function.

---

### Lesson 18 — The ? Operator
**Learn:** `?` early-return on Err when both calls share the same error type
**Exercise:** Parse two numbers and return them as a tuple using `?` twice.
**You're done when:** You can chain fallible calls with `?` without `match`.

---

### Lesson 19 — Error Chaining with `map_err`
**Learn:** `.map_err()` to convert error types, building friendly error strings
**Exercise:** Parse two strings as `f64` and divide them, handling all error cases with descriptive messages.
**You're done when:** You can bridge different error types into a single `Result`.

---

### Lesson 20 — Closures
**Learn:** Anonymous functions (`|x| x + 1`), capturing environment, `impl Fn(...)`
**Exercise:** Write an `apply(x, f)` function that runs a closure on a value.
**You're done when:** You can pass closures around and use ones that capture local state.

---

### Lesson 21 — Map + Collect
**Learn:** `.map().collect()` — transforming an iterator into a new `Vec`
**Exercise:** Double every number in a slice and collect into a Vec.
**You're done when:** You can turn an iterator chain into a concrete collection with `.collect()`.

---

### Lesson 22 — Filter
**Learn:** `.filter()` — selecting elements from an iterator; the `&&n` double-reference pattern
**Exercise:** Return the squares of the even numbers in a slice.
**You're done when:** You understand why `.filter()` gives `&&i32` and can destructure it.

---

### Lesson 23 — Sum
**Learn:** `.sum()` — the simplest aggregator, collapsing an iterator into one value
**Exercise:** Sum all integers in a slice.
**You're done when:** You can aggregate an iterator without a manual loop.

---

### Lesson 24 — Fold
**Learn:** `.fold(init, |acc, &n| ...)` — general-purpose reduction
**Exercise:** Compute the product of all integers in a slice.
**You're done when:** You can express any accumulation pattern with `.fold()`.

---

### Lesson 25 — Debug Format
**Learn:** `{:?}` vs `{}`, `format!("{:?}", ...)`, how `#[derive(Debug)]` connects to traits
**Exercise:** Return the debug representation of a slice as a String.
**You're done when:** You know when to use `{:?}` and why `Vec` can't use `{}`.

---

### Lesson 26 — Traits
**Learn:** `trait` definition, `impl Trait for Type`, `&impl Trait` arguments
**Exercise:** Define a `Describable` trait and implement it for an `Item` struct.
**You're done when:** You can define a trait and write a function that accepts any type implementing it.

---

### Lesson 27 — Generics
**Learn:** Type parameters `<T>` on functions, impl blocks, and structs
**Exercise:** Write a generic `swap<T>(a, b)` that returns `(b, a)` for any type.
**You're done when:** You understand what you can and can't do with an unconstrained `T`.

---

### Lesson 28 — Trait Bounds
**Learn:** `T: Ord` — constraining generics so you can compare, copy, or use specific methods
**Exercise:** Write `larger<T: Ord>(a, b) -> T` that returns the bigger of two values.
**You're done when:** You can pick the minimum bounds needed for a generic function.

---

### Lesson 29 — Lifetimes Intro
**Learn:** `'a` annotations, why the borrow checker needs them, elision rules
**Exercise:** Write `longer<'a>(a: &'a str, b: &'a str) -> &'a str` that returns the longer slice.
**You're done when:** You can annotate a function that returns a borrowed reference.

---

### Lesson 30 — `Box<T>` — Heap Allocation
**Learn:** `Box::new()`, heap allocation, auto-deref
**Exercise:** Write `boxed_price(cents) -> Box<i32>` to heap-allocate a salon service price.
**You're done when:** You can explain when boxing is necessary (recursive types, trait objects).

---

### Lesson 31 — `Rc<T>` — Shared Ownership
**Learn:** Reference-counted shared ownership, `Rc::clone()`, `Rc::strong_count()`
**Exercise:** 3 stations share one price list via `Rc` — return the strong count.
**You're done when:** You can share data between multiple owners without the compiler complaining.

---

### Lesson 32 — Modules and Visibility
**Learn:** `mod`, `pub`, `use`, file-based modules via `#[path]`
**Exercise:** Complete a `ClientCounter` walk-in tracker in a separate module.
**You're done when:** You can organise code across files and control visibility with `pub`.

---

### Lesson 33 — Capstone: Service Log
**Learn:** Everything combined — modules + HashMap + Vec + Option + methods
**Exercise:** Complete a `ServiceLog` — track revenue per technician (Mai, Linh, Trang).
**You're done when:** All 33 core lesson tests pass.

---

### Lesson 34 — Custom Error Enum
**Learn:** `enum BookingError { SlotTaken, TechnicianOff }` — typed error variants
**Exercise:** Write `validate_booking()` returning the correct error variant.
**You're done when:** You can define and return custom errors from functions.

---

### Lesson 35 — `impl Display` for Errors
**Learn:** `fmt::Display` trait — how errors become human-readable strings
**Exercise:** Implement `Display` for `BookingError` with descriptive messages.
**You're done when:** Your errors print friendly messages, not just `{:?}` debug output.

---

### Lesson 36 — `impl Error` Trait
**Learn:** `std::error::Error` — the trait that makes errors composable
**Exercise:** Wire `BookingError` into `Box<dyn Error>` by implementing the Error trait.
**You're done when:** Your custom error works anywhere `Box<dyn Error>` is expected.

---

### Lesson 37 — `thiserror` Derive
**Learn:** `#[derive(thiserror::Error)]` — the boilerplate vanishes
**Exercise:** Replace manual Display + Error impls with `thiserror` derive macros.
**You're done when:** You see the same behavior with a fraction of the code.

---

### Lesson 38 — `anyhow` Catch-All
**Learn:** `anyhow::Result`, `bail!`, `.context()` — one Result type for everything
**Exercise:** Write `process_booking()` that can fail 3 ways — `anyhow` catches all.
**You're done when:** You can handle mixed error types without custom enums.

---

### Lesson 39 — Slices `&[T]`
**Learn:** Explicit slices, DST semantics, `&vec[..]`
**Exercise:** Write `daily_revenue(prices: &[u32]) -> u32` — sum a slice of service prices.
**You're done when:** You can accept and operate on slices without owning the data.

---

### Lesson 40 — Struct Lifetimes
**Learn:** `struct Appointment<'a>` — structs that borrow data
**Exercise:** Create an `Appointment` with borrowed client and service names.
**You're done when:** You can embed references in structs with lifetime annotations.

---

### Lesson 41 — Lifetime Elision
**Learn:** When Rust infers lifetimes for you — elision rules on methods
**Exercise:** Write methods on `Appointment` that return `&str` without explicit `'a`.
**You're done when:** You know when you can omit lifetime annotations.

---

### Lesson 42 — `Cell<T>`
**Learn:** Interior mutability with `Cell` — mutate through `&self`
**Exercise:** Build a `TipJar` where `add()` takes `&self` (not `&mut self`).
**You're done when:** You can mutate `Copy` types through shared references.

---

### Lesson 43 — `RefCell<T>`
**Learn:** Runtime borrow checking with `RefCell` — `borrow()` / `borrow_mut()`
**Exercise:** Build a `Schedule` that pushes appointments through `&self`.
**You're done when:** You can mutate non-Copy types through shared references.

---

### Lesson 44 — `Rc<RefCell<T>>`
**Learn:** Shared + mutable — the `Rc<RefCell<T>>` pattern
**Exercise:** Two stations share and mutate one appointment book.
**You're done when:** You can share mutable state between multiple owners.

---

### Lesson 45 — Custom Iterator
**Learn:** `impl Iterator for MyType` — the `next()` method
**Exercise:** Implement `Iterator` for `WalkInQueue` — yields waiting clients one by one.
**You're done when:** You can `for client in queue` over your custom type.

---

### Lesson 46 — Iterator Adaptors on Custom Type
**Learn:** `.filter()` and `.map()` work on any `Iterator` — including yours
**Exercise:** Filter `WalkInQueue` by service type, map to prices.
**You're done when:** You can chain standard adaptors on your custom iterator.

---

### Lesson 47 — CSV Read
**Learn:** `csv::Reader` + `#[derive(Deserialize)]` — parse CSV rows into structs
**Exercise:** Write `load_price_list(path)` that reads a `name,price` CSV into a `Vec<Service>`.
**You're done when:** You can turn a CSV file into typed structs — no `splitn`/`parse` by hand.

---

### Lesson 48 — CSV Write
**Learn:** `csv::Writer` + `#[derive(Serialize)]` — write structs as CSV rows
**Exercise:** Write `save_daily_log(path, entries)` — serialize `Service` rows to a CSV (the header is written automatically).
**You're done when:** You can persist typed structs as a CSV without building strings by hand.

---

### Lesson 49 — Serde JSON
**Learn:** `#[derive(Serialize, Deserialize)]` — automatic JSON conversion
**Exercise:** Serialize and deserialize a `ServiceEntry` struct as JSON.
**You're done when:** You can round-trip Rust structs through JSON.

---

### Lesson 50 — `async fn` + tokio
**Learn:** `async fn`, `.await`, `#[tokio::main]`
**Exercise:** Write `check_availability(slot)` — ask the clinic scheduler whether a slot is free. Your first async function.
**You're done when:** You can write and await async functions.

---

### Lesson 51 — `tokio::spawn`
**Learn:** Spawning concurrent tasks
**Exercise:** Two patients booked at once — spawn tasks, collect results.
**You're done when:** You can run independent work concurrently with `tokio::spawn`.

---

### Lesson 52 — Async Channels
**Learn:** `tokio::sync::mpsc` — message passing between tasks
**Exercise:** Nurses report "done" on a channel as their rounds finish; the nurses' station receives.
**You're done when:** You can communicate between async tasks.

---

### Lesson 53 — `clap` Arg Parsing
**Learn:** `#[derive(Parser)]` — structured CLI argument parsing
**Exercise:** Parse a medication order: `--patient "Mr. Hung" --medication Paracetamol --dose-mg 500`.
**You're done when:** You can build type-safe CLI interfaces.

---

### Lesson 54 — Capstone: Ward CLI
**Learn:** Everything combined — structs, HashMap, methods, CLI
**Exercise:** A ward care log: log visits, list them, total care minutes by nurse.
**You're done when:** The care log runs end to end — log, list, and report minutes per nurse.

---

## RUST GENERAL HOSPITAL (c55–c74)

From c50 onward, every lesson is set at Rust General Hospital: one small, real job that hospital software does — no story arc, just a reason for the concept to exist. Mai, Linh, and Trang are the nurses; the ⚡ warmups use the hospital's medicine-delivery robot. ⚡ = warmup (c01–c54 tools only). ★ = BUG HUNT: the exercise ships broken code that compiles — you diagnose and fix it.

---

**Ward Equipment (c55–c58):** the delivery robot, the bedside monitor, and an infusion pump — `Box`, trait objects, and `Deref`.

### Lesson 55 — ⚡ Climbing Stairs (Warmup: Fibonacci)
**Learn:** Iterative computation with a rolling pair — no recursion needed
**Exercise:** `climb_ways(steps)` — how many ways the delivery robot can climb a staircase taking 1 or 2 steps at a time — plus `ways_table(len)`.
**You're done when:** You can turn a recurrence into a loop with `(a, b) = (b, a + b)`.

---

### Lesson 56 — Delivery Route (Recursive Types with Box)
**Learn:** Why a self-referential `enum` needs `Box` (it would have infinite size otherwise)
**Exercise:** Build the robot's `Route` cons-list: `build_route`, `stop_count`, and `last_stop` — the last stop before the dock.
**You're done when:** You understand that `Box` gives a recursive type a fixed, known size.

---

### Lesson 57 — Bedside Monitor (Box\<dyn\>)
**Learn:** `Box<dyn Trait>`, dynamic dispatch, heterogeneous collections
**Exercise:** Implement `Sensor` for `HeartRate`/`BloodOxygen`/`BloodPressure`, rack all three in one `Vec<Box<dyn Sensor>>`, total the power draw.
**You're done when:** You can store different concrete types together and call shared methods at runtime.

---

### Lesson 58 — ★ Infusion Pump (Deref)
**Learn:** `impl Deref`, the `*` operator, and auto-deref coercion
**Exercise:** BUG HUNT — every pump reads its factory default even though a nurse programmed it. Find which field `Deref` serves.
**You're done when:** `two_hour_volume(&pump)` reaches the programmed rate through auto-deref — the pump reads 42 ml over two hours.

---

**Safe Shutdown (c59–c61):** cleanup that always runs, and links that let go.

### Lesson 59 — ⚡ Last Trip (Warmup: Two Sum)
**Learn:** One-pass hashing — trade space for time
**Exercise:** `trip_pair(legs, charge)` — each number is the battery cost of one hallway; return the positions of two hallways whose costs add up to exactly the robot's remaining charge.
**You're done when:** You can find a complement in O(n) instead of a nested loop.

---

### Lesson 60 — Pump Stop (Drop / RAII)
**Learn:** `impl Drop`, deterministic cleanup, reverse (LIFO) drop order
**Exercise:** Make every `PumpSession` push "&lt;bed&gt; pump stopped" on drop; stop a blocked line early with `drop()`.
**You're done when:** Cleanup runs automatically when a value leaves scope — no GC, no forgotten close.

---

### Lesson 61 — ★ Bed Alarm (Weak\<T\>)
**Learn:** `Rc::downgrade`, `Weak::upgrade`, why cycles leak and how `Weak` breaks them
**Exercise:** BUG HUNT — the bed alarm's strong `Rc` grip keeps a discharged patient's record alive, and the alarm keeps ringing. Break the cycle.
**You're done when:** `link_counts()` is `(1, 1)` and after discharge the alarm's `upgrade()` returns `None`.

---

**Shared Care (c62–c64):** interior mutability on shared equipment and shared charts.

### Lesson 62 — ⚡ Retrace (Warmup: Reverse in Place)
**Learn:** In-place mutation with two pointers
**Exercise:** `retrace` — reverse the robot's path back to the pharmacy by swapping `i` with `n-1-i`, no allocation.
**You're done when:** You can mutate a collection in place without allocating a new one.

---

### Lesson 63 — Crash Cart (Cell\<T\> Full API)
**Learn:** `Cell::replace` and `Cell::take` — mutate `Copy` data through `&self`
**Exercise:** The shared defibrillator: `recharge` (set a new charge, return the previous one) and `shock` (deliver it all, leave 0).
**You're done when:** You can swap interior state through a shared reference and recover the old value.

---

### Lesson 64 — ★ Chart Under Review (RefCell Runtime Borrow)
**Learn:** Runtime borrow checking — `borrow_mut` panics, `try_borrow_mut` returns `Err`
**Exercise:** BUG HUNT — adding a note while a doctor reviews the chart crashes the charting system. Refuse the contended write gracefully.
**You're done when:** Contended writes come back `Err`, writes after the review succeed, nothing panics.

---

**Night Shift (c65–c68):** many nurses, many threads, one set of records.

### Lesson 65 — ⚡ Double Dose (Warmup: Contains Duplicate)
**Learn:** `HashSet` membership — a HashMap with keys only
**Exercise:** `double_delivery` — the robot catches a medication order delivered twice, in one pass.
**You're done when:** You can use `HashSet::insert`'s return value to detect duplicates.

---

### Lesson 66 — Supply Check (Arc\<T\> Across Threads)
**Learn:** `Arc`, `thread::spawn`, `join` — sharing immutable data across threads
**Exercise:** `supply_check` — three nurses' threads each double-check one shared supply list.
**You're done when:** You understand why threads need `Arc` (it's `Send`) and `Rc` won't compile.

---

### Lesson 67 — ★ The Missing Fluids (Arc\<Mutex\<T\>\>)
**Learn:** `Mutex` for thread-safe mutation — the threaded sibling of `Rc<RefCell<T>>`
**Exercise:** BUG HUNT — ten nurses chart 100 ml each; the fluid total reads 0. The compiler is waving a warning at the culprit.
**You're done when:** `fluid_total()` reads 1000 every run — entries go through the lock, not into a copy.

---

### Lesson 68 — Bed Board (RwLock\<T\>)
**Learn:** `RwLock` — many concurrent readers OR one exclusive writer
**Exercise:** `free_beds_seen` — the charge nurse writes the bed board, three nurses read it concurrently.
**You're done when:** You can share data across threads with read/write locks.

---

**Pharmacy (c69–c74):** persistent stock records, and a capstone that runs on your own code.

### Lesson 69 — Stock Records (Sled Insert)
**Learn:** `sled::open` + `db.insert` — a pure-Rust embedded key-value database, no SQL
**Exercise:** `store(db, key, record)` — every stock record goes straight to disk.
**You're done when:** You can open a sled database and write entries that survive a restart.

---

### Lesson 70 — Stock Lookup (Sled Get)
**Learn:** `db.get` → `Option<IVec>` — reading values back by key
**Exercise:** `fetch(db, key)` — look a drug up; a drug the pharmacy doesn't stock is `None`, not a crash.
**You're done when:** You can read values out of the store and handle the missing-key case.

---

### Lesson 71 — Stock Codec (Sled + Serde)
**Learn:** store whole structs — `serde_json::to_vec` / `from_slice` (c49), errors unified with `anyhow` (c38)
**Exercise:** `encode_stock` / `decode_stock` — round-trip `Stock { drug, units }` through the database.
**You're done when:** Structs persist and reload intact. (The c74 capstone imports THIS file — build it well.)

---

### Lesson 72 — Stock Take (Sled Iterate)
**Learn:** `db.iter()` — scanning every entry in the store
**Exercise:** `full_inventory(db)` — list everything the pharmacy holds, sorted by drug name.
**You're done when:** You can walk the whole database and decode each record.

---

### Lesson 73 — Reorder List (Sled Query)
**Learn:** "querying" without SQL — `filter` + `map` over the records (c21-22 / c45-46)
**Exercise:** `low_stock(db, below)` and `low_stock_names` — every drug running below the threshold.
**You're done when:** You can answer questions about the data with iterator combinators.

---

### Lesson 74 — CAPSTONE: Delivery CSV → Pharmacy
**Learn:** integration — read a CSV (c47), store it via YOUR c71 codec (imported with `#[path]`), report on the stock
**Exercise:** `import_delivery(db, csv_path)` totals the units on hand; `most_stocked(db)` finds the best-stocked drug.
**You're done when:** The stock totals 12450, Paracetamol is the best stocked, and the run prints PHARMACY ONLINE.

---

## Bug Hunt (c75–c80)

Side jobs around the hospital: the back-office code is broken, and you debug it. Every ★ exercise COMPILES but fails its tests — read the `// BUG:` symptom at the top of the file, run the tests, diagnose, fix. No new concepts (everything is c01–c54 material); the example file holds the corrected reference.

---

### Lesson 75 — Bug Hunt: The Vanishing Tally
**Learn:** Why counting needs `entry(k).or_insert(0)` — `insert` overwrites instead of accumulating
**Exercise:** Mai's visit count is stuck at 1; fix `visit_counts` so tallies accumulate.
**You're done when:** Mai reads 6, Linh 2, Trang 1, and the empty shift yields an empty map.

---

### Lesson 76 — Bug Hunt: The Silent Zero
**Learn:** Don't swallow parse errors — a bad row must propagate as `Err`, not become 0
**Exercise:** A smudged fluid-chart entry silently undercounts the patient's intake; make `fluid_intake` report it.
**You're done when:** A clean chart sums correctly and a corrupt row returns `Err`.

---

### Lesson 77 — Bug Hunt: The Missing Critical Beds
**Learn:** `filter` keeps the items whose predicate is true — direction matters
**Exercise:** The oxygen report totals the stable beds instead of the critical ones; fix the filter.
**You're done when:** `critical_oxygen_total` sums only critical beds (30) and 0 when there are none.

---

### Lesson 78 — Bug Hunt: The Overlooked Rush
**Learn:** The last window starts at `n - width`, so the loop must be inclusive (`..=`)
**Exercise:** The ER's busiest-3-hour report always misses the late-night rush; fix the bound.
**You're done when:** The busiest window is found even when it's the last one.

---

### Lesson 79 — Bug Hunt: The Double-Booked Borrow
**Learn:** A `borrow()` guard blocks `borrow_mut()` until it's dropped — scope it tightly
**Exercise:** Booking a new scan panics with BorrowMutError; release the read borrow first.
**You're done when:** `add_if_absent` adds new bookings, rejects duplicates, and never panics.

---

### Lesson 80 — Bug Hunt: The Half-Heard Clock-Out
**Learn:** `while let Some(msg) = rx.recv().await` drains a channel; one `if let` reads only one
**Exercise:** Three nurses clock out but only Mai is logged; drain the whole channel.
**You're done when:** `collect_done` returns all three "done" messages in order.

---

### Beyond Chapter 80
The lessons above cover the core language, practical application, and debugging reps. Next topics:
- Procedural and derive macros
- Advanced async (`select!`, `RwLock`, cancellation)
- Web server with `axum` or `actix-web`
- Relational/SQL databases with `sqlx` or `SeaORM` (you've done embedded key-value with `sled`)
- Building the ward care log as a web API
