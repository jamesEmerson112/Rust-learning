# Rust Learning — Roadmap

**74 lessons, ~15 minutes each, one new concept per lesson.** (⚡ = light DSA warmup)

Each lesson is a triple:
- `src/bin/cXX_example.rs` — complete reference (read it)
- `src/bin/cXX_exercise.rs` — stubs with `TODO`s (fill it in)
- `tests/cXX_tests.rs` — integration tests (run until green)

Run the tracker: `cargo run --bin progress`

---

## At a Glance

| #  | Title                                 | Key concept                              |
|----|---------------------------------------|------------------------------------------|
| 01 | Hello Variables                       | `let`, `mut`, primitive types            |
| 02 | Strings and Formatting                | `&str` vs `String`, `format!`            |
| 03 | Arrays and Iteration                  | `[T; N]`, `.iter()`                      |
| 04 | Tuples and Type Casting               | tuple returns, `as`                      |
| 05 | If/Else and For Loops                 | control flow, `Vec::new()`               |
| 06 | Match and String Building             | `match`, `.join()`                       |
| 07 | Ownership and Borrowing               | move semantics, `&T`                     |
| 08 | String Slices and Methods             | `&str` views, iterator chaining          |
| 09 | Structs                               | `struct`, `#[derive(Debug)]`             |
| 10 | Methods and impl Blocks               | `impl`, `&self`                          |
| 11 | Enums                                 | variants with data                       |
| 12 | `Option<T>`                           | `Some`/`None`, `if let`                  |
| 13 | HashMap Basics                        | `.insert()`, `.get()`                    |
| 14 | HashMap Entry API                     | `.entry().or_insert()`                   |
| 15 | String Normalization                  | `.trim_matches()`, `.to_lowercase()`     |
| 16 | Word Count (HashMap + Strings)        | composing text pipelines                 |
| 17 | Result Basics                         | `Result<T,E>`, `match` on Result         |
| 18 | The `?` Operator                      | early-return on `Err`                    |
| 19 | Error Chaining with `map_err`         | bridging error types                     |
| 20 | Closures                              | `|x| expr`, capture, `impl Fn`           |
| 21 | Map + Collect                         | `.map().collect()` — iterator to Vec     |
| 22 | Filter                                | `.filter()` + the `&&` double-reference  |
| 23 | Sum                                   | `.sum()` — simplest aggregator           |
| 24 | Fold                                  | `.fold(init, \|acc, &n\| ...)` reduction |
| 25 | Debug Format                          | `{:?}` vs `{}`, `format!("{:?}", ...)`   |
| 26 | Traits                                | `trait` + `impl Trait for Type`          |
| 27 | Generics                              | `<T>`                                    |
| 28 | Trait Bounds                          | `T: Ord` constrains generics             |
| 29 | Lifetimes Intro                       | `'a` on fn signatures                    |
| 30 | `Box<T>` — Heap Allocation            | `Box::new`, salon service prices         |
| 31 | `Rc<T>` — Shared Ownership            | `Rc::clone`, shared station price lists  |
| 32 | Modules and Visibility                | `mod`, `pub`, `ClientCounter`            |
| 33 | Capstone: Service Log                 | `ServiceLog` — revenue per technician    |
| 34 | Custom Error Enum                     | `enum BookingError`, `validate_booking()`|
| 35 | `impl Display` for Errors             | human-readable error messages            |
| 36 | `impl Error` Trait                    | wire into `std::error::Error`            |
| 37 | `thiserror` Derive                    | `#[derive(Error)]` — no boilerplate      |
| 38 | `anyhow` Catch-All                    | `anyhow::Result` for mixed errors        |
| 39 | Slices `&[T]`                         | `daily_revenue(prices: &[u32])`          |
| 40 | Struct Lifetimes                      | `Appointment<'a>` borrows client data    |
| 41 | Lifetime Elision                      | methods returning `&str` — Rust infers   |
| 42 | `Cell<T>`                             | `TipJar` — mutate through `&self`        |
| 43 | `RefCell<T>`                          | `Schedule` — push through `&self`        |
| 44 | `Rc<RefCell<T>>`                      | two stations share one appointment book  |
| 45 | Custom Iterator                       | `WalkInQueue` yields client names        |
| 46 | Iterator Adaptors                     | filter + map on `WalkInQueue`            |
| 47 | CSV Read                              | `load_price_list()` parses services.csv  |
| 48 | CSV Write                             | `save_daily_log()` writes a CSV          |
| 49 | Serde JSON                            | `ServiceEntry` serialize/deserialize     |
| 50 | `async fn` + tokio                    | `check_availability()` with `.await`     |
| 51 | `tokio::spawn`                        | concurrent bookings                      |
| 52 | Async Channels                        | technician sends "done" on `mpsc`        |
| 53 | `clap` Arg Parsing                    | `--technician --service --price`         |
| 54 | Capstone: Salon CLI                   | full scheduler — book, list, revenue     |
| ⚡ 55 | Warmup: Fibonacci                  | iterative loop, tuple swap               |
| 56 | Recursive Types with Box              | `enum` + `Box` for self-referential data |
| 57 | Trait Objects `Box<dyn>`              | dynamic dispatch over mixed types        |
| 58 | Deref — Custom Smart Pointer          | `impl Deref`, auto-deref coercion        |
| ⚡ 59 | Warmup: Two Sum                    | one-pass HashMap complement lookup       |
| 60 | `Drop` (RAII)                         | deterministic cleanup, LIFO drop order   |
| 61 | `Weak<T>` and Cycles                  | `downgrade`/`upgrade`, leak-proof refs   |
| ⚡ 62 | Warmup: Reverse a Vec             | two-pointer in-place swap                |
| 63 | `Cell<T>` Full API                    | `replace` / `take` through `&self`       |
| 64 | RefCell Runtime Borrow                | `try_borrow_mut`, runtime borrow check   |
| ⚡ 65 | Warmup: Contains Duplicate        | HashSet membership in one pass           |
| 66 | `Arc<T>` Across Threads               | shared immutable state, `thread::spawn`  |
| 67 | `Arc<Mutex<T>>`                       | shared mutable state across threads      |
| 68 | `RwLock<T>`                           | many readers / one writer                |
| 69 | `sled` insert                         | open a DB, insert key/value bytes        |
| 70 | `sled` get                            | read values back by key                  |
| 71 | `sled` + serde                        | store & load structs with serde          |
| 72 | `sled` iterate                        | scan every entry in the store            |
| 73 | `sled` query                          | filter + map records, no SQL             |
| 74 | CSV → `sled`                          | import a CSV, then aggregate             |

See [README.md](README.md) Study Plan for the **Learn / Exercise / Done-when** detail on every lesson.

---

## Phases

**Foundations (c01-c06)** — syntax, control flow, primitives. How to write a compilable Rust program.

**Ownership Era (c07-c08)** — the one Rust idea other languages don't have. Everything downstream builds on this.

**Data Modeling (c09-c16)** — structs, enums, `Option`, `HashMap`, strings. Shape your data.

**Error Handling I (c17-c19)** — `Result`, then `?`, then `map_err`. Each lesson answers the question the previous one raised.

**Functional Rust (c20-c24)** — closures are the prerequisite for iterator combinators, which unfold one at a time: collect, filter, sum, fold.

**Debugging (c25)** — the `{:?}` format and the Debug trait. Bridges naturally into "what IS a trait?"

**Abstractions (c26-c29)** — traits define contracts, generics consume them, bounds constrain them, lifetimes borrow through them.

**Heap & Sharing (c30-c31)** — `Box` for single owners, `Rc` for many. Salon-themed: service prices on the heap, shared price lists.

**Code Organization (c32-c33)** — modules + capstone. `ClientCounter` walk-in tracker, `ServiceLog` revenue tracker.

**Error Handling II (c34-c38)** — custom error enums, Display, Error trait, then see the boilerplate vanish with `thiserror` and `anyhow`. All themed around salon booking errors.

**Slices & Lifetimes II (c39-c41)** — explicit `&[T]` slices, struct lifetimes with `Appointment<'a>`, lifetime elision on methods.

**Interior Mutability (c42-c44)** — `Cell<T>` tip jar, `RefCell<T>` schedule, `Rc<RefCell<T>>` shared appointment book.

**Custom Iterators (c45-c46)** — `impl Iterator` for `WalkInQueue`, then filter/map adaptors on it.

**File I/O (c47-c49)** — read a CSV price list and write a CSV daily log with the `csv` crate, then serialize/deserialize with serde JSON.

**Async (c50-c52)** — first `async fn` + `.await`, then `tokio::spawn` for concurrent bookings, then `mpsc` channels.

**CLI Project (c53-c54)** — `clap` arg parsing, then the capstone: a full salon scheduler CLI.

**DSA Warmups (c55, c59, c62, c65)** — light, recognizable interview problems (Fibonacci, Two Sum, reverse-in-place, contains-duplicate) interleaved as a gentle on-ramp before each smart-pointer cluster. Solvable with everything from c01-c54 — pure confidence reps, no new syntax.

**Box, Properly (c56-c58)** — the two real reasons `Box` exists: recursive types and trait objects (`Box<dyn Trait>`), then `Deref` to demystify how every smart pointer works.

**Lifecycle & Cycles (c60-c61)** — `Drop` for RAII cleanup, then `Weak<T>` to break the reference cycles that would otherwise leak.

**Interior Mutability, Deeper (c63-c64)** — the full `Cell` API, then deliberately tripping `RefCell`'s runtime borrow check to feel the compile-time-vs-runtime trade.

**Concurrency Siblings (c66-c68)** — `Arc` to share across threads, `Arc<Mutex<T>>` (the threaded mirror of c44's `Rc<RefCell<T>>`), and `RwLock` for many-readers/one-writer.

---

## Prerequisite Chain

```
c01 → c02 → c03 → c04 → c05 → c06
                                │
                                ▼
                               c07 (ownership) ──────────┐
                                │                        │
                                ▼                        │
                               c08 (slices)              │
                                │                        │
                ┌───────────────┴──────────┐             │
                ▼                          ▼             │
               c09 (struct) → c10 (impl)  c11 (enum) → c12 (Option)
                │                                        │
                └──────────┬─────────────────────────────┘
                           ▼
                          c13 (HashMap) → c14 (entry) → c15 (normalize) → c16 (word count)
                                                                           │
                                                                           ▼
                                                                          c17 (Result) → c18 (?) → c19 (map_err)
                                                                                                     │
                                                                                                     ▼
                                                                                                    c20 (closures)
                                                                                                     │
                                                                                                     ▼
                                                          c21 (collect) → c22 (filter) → c23 (sum) → c24 (fold)
                                                                                                       │
                                                                                                       ▼
                                                                                                      c25 (debug)
                                                                                                       │
                                                                                                       ▼
                                                          c26 (traits) → c27 (generics) → c28 (bounds) → c29 (lifetimes)
                                                                                                            │
                                                                                                            ▼
                                                                                               c30 (Box) → c31 (Rc) → c32 (modules) → c33 (capstone)
                                                                                                                                          │
                                                                                                                                          ▼
                                                                          c34 (error enum) → c35 (Display) → c36 (Error) → c37 (thiserror) → c38 (anyhow)
                                                                                                                                                  │
                                                                                                                                                  ▼
                                                                                                          c39 (slices) → c40 (struct lifetimes) → c41 (elision)
                                                                                                                                                     │
                                                                                                                                                     ▼
                                                                                                               c42 (Cell) → c43 (RefCell) → c44 (Rc<RefCell>)
                                                                                                                                                  │
                                                                                                                                                  ▼
                                                                                                                     c45 (custom iter) → c46 (adaptors)
                                                                                                                                              │
                                                                                                                                              ▼
                                                                                                     c47 (csv read) → c48 (csv write) → c49 (serde)
                                                                                                                                              │
                                                                                                                                              ▼
                                                                                               c50 (async fn) → c51 (spawn) → c52 (channels)
                                                                                                                                   │
                                                                                                                                   ▼
                                                                                                              c53 (clap) → c54 (salon CLI capstone)
                                                                                                                                        │
                                                                                                                                        ▼
            ⚡c55 (warmup) → c56 (recursive Box) → c57 (Box<dyn>) → c58 (Deref)
                                                                       │
                                                                       ▼
            ⚡c59 (warmup) → c60 (Drop) → c61 (Weak)
                                             │
                                             ▼
            ⚡c62 (warmup) → c63 (Cell API) → c64 (RefCell borrow)
                                                  │
                                                  ▼
            ⚡c65 (warmup) → c66 (Arc) → c67 (Arc<Mutex>) → c68 (RwLock)
                                                  │
                                                  ▼
            c69 (insert) → c70 (get) → c71 (serde) → c72 (iterate) → c73 (query) → c74 (CSV→sled)
```

---

## Post-74 Track

The 74-lesson curriculum covers the core language plus practical application. Once you clear c74, good next topics:

1. **Procedural and derive macros** — `macro_rules!`, `#[derive(...)]` custom macros.
2. **Advanced async** — `select!`, `tokio::sync::RwLock`, cancellation, backpressure.
3. **Web server** — `axum` or `actix-web` with the salon scheduler as an API.
4. **Relational / SQL databases** — `sqlx` or `SeaORM` over SQLite/Postgres (you've done embedded key-value with `sled`; next is querying a SQL engine).
5. **Testing patterns** — property-based testing, mocking, integration test organization.

---

## One Rule

**One new concept per lesson.** If a lesson introduces more than one new thing, it's a bug — flag it and we'll split it.
