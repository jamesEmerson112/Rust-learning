# Rust Learning

See [ROADMAP.md](ROADMAP.md) for a scannable overview of all 54 lessons.

This repo contains small Rust lessons, each with:
- an `example` binary (complete reference)
- an `exercise` binary (starter stubs with `TODO`s)
- an integration test file for validation

## File layout
- `src/bin/c01_example.rs` ... `src/bin/c54_example.rs`
- `src/bin/c01_exercise.rs` ... `src/bin/c54_exercise.rs`
- `tests/c01_tests.rs` ... `tests/c54_tests.rs`

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

### Lesson 47 — File Read
**Learn:** `fs::read_to_string` — loading data from disk
**Exercise:** Write `load_price_list(path)` that reads a CSV-style services file.
**You're done when:** You can read files and parse structured text data.

---

### Lesson 48 — File Write
**Learn:** `fs::write` — persisting data to disk
**Exercise:** Write `save_daily_log(path, entries)` — output today's services to a file.
**You're done when:** You can write formatted data to files with error handling.

---

### Lesson 49 — Serde JSON
**Learn:** `#[derive(Serialize, Deserialize)]` — automatic JSON conversion
**Exercise:** Serialize and deserialize a `ServiceEntry` struct as JSON.
**You're done when:** You can round-trip Rust structs through JSON.

---

### Lesson 50 — `async fn` + tokio
**Learn:** `async fn`, `.await`, `#[tokio::main]`
**Exercise:** Write `check_availability(slot)` — your first async function.
**You're done when:** You can write and await async functions.

---

### Lesson 51 — `tokio::spawn`
**Learn:** Spawning concurrent tasks
**Exercise:** Two clients booking concurrently — spawn tasks, collect results.
**You're done when:** You can run independent work concurrently with `tokio::spawn`.

---

### Lesson 52 — Async Channels
**Learn:** `tokio::sync::mpsc` — message passing between tasks
**Exercise:** Technician sends "done" on a channel, front desk receives.
**You're done when:** You can communicate between async tasks.

---

### Lesson 53 — `clap` Arg Parsing
**Learn:** `#[derive(Parser)]` — structured CLI argument parsing
**Exercise:** Parse `--technician "Mai" --service "Gel Manicure" --price 45`.
**You're done when:** You can build type-safe CLI interfaces.

---

### Lesson 54 — Capstone: Salon CLI
**Learn:** Everything combined — structs, HashMap, methods, CLI
**Exercise:** Full salon scheduler: book, list, revenue by technician.
**You're done when:** All 54 lesson tests pass. You ship software.

---

### Beyond Chapter 54
The lessons above cover the core language plus practical application. Next topics:
- Procedural and derive macros
- Advanced async (`select!`, `RwLock`, cancellation)
- Web server with `axum` or `actix-web`
- Database with `sqlx` or `diesel`
- Building the salon scheduler as a web API
