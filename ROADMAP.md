# Rust Learning — Roadmap

**80 lessons, ~15 minutes each, one new concept per lesson.** (⚡ = light DSA warmup, ★ = fix-the-bug: broken code you debug instead of stubs you fill in)

From c50 onward every lesson is set at **Rust General Hospital** — one small, real job that hospital software does per lesson, no story arc. c55–c74 are grouped into five chapters; c75–c80 are **Bug Hunt** side jobs around the hospital: pure debugging practice on c01–c54 material.

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
| 51 | `tokio::spawn`                        | two patients booked concurrently         |
| 52 | Async Channels                        | nurses send "done" on `mpsc`             |
| 53 | `clap` Arg Parsing                    | `--patient --medication --dose-mg`       |
| 54 | Capstone: Ward CLI                    | care log — log visits, list, minutes     |
| ⚡ 55 | Climbing Stairs (Warmup: Fibonacci) | iterative loop, tuple swap              |
| 56 | Delivery Route (recursive Box)        | `enum` + `Box` for self-referential data |
| 57 | Bedside Monitor (`Box<dyn>`)          | dynamic dispatch over mixed types        |
| ★ 58 | Infusion Pump (Deref)               | `impl Deref`, auto-deref coercion        |
| ⚡ 59 | Last Trip (Warmup: Two Sum)        | one-pass HashMap complement lookup       |
| 60 | Pump Stop (`Drop`/RAII)               | deterministic cleanup, LIFO drop order   |
| ★ 61 | Bed Alarm (`Weak<T>`)               | `downgrade`/`upgrade`, leak-proof refs   |
| ⚡ 62 | Retrace (Warmup: Reverse)          | two-pointer in-place swap                |
| 63 | Crash Cart (`Cell<T>` full API)       | `replace` / `take` through `&self`       |
| ★ 64 | Chart Under Review (RefCell borrow) | `try_borrow_mut`, runtime borrow check   |
| ⚡ 65 | Double Dose (Warmup: Dupes)        | HashSet membership in one pass           |
| 66 | Supply Check (`Arc<T>` across threads)| shared immutable state, `thread::spawn`  |
| ★ 67 | The Missing Fluids (`Arc<Mutex<T>>`)| shared mutable state across threads      |
| 68 | Bed Board (`RwLock<T>`)               | many readers / one writer                |
| 69 | Stock Records (`sled` insert)         | open a DB, insert key/value bytes        |
| 70 | Stock Lookup (`sled` get)             | read values back by key                  |
| 71 | Stock Codec (`sled` + serde)          | store & load structs with serde          |
| 72 | Stock Take (`sled` iterate)           | scan every entry in the store            |
| 73 | Reorder List (`sled` query)           | filter + map records, no SQL             |
| 74 | CAPSTONE: Delivery CSV → Pharmacy     | capstone assembles YOUR c71 codec        |
| ★ 75 | Bug Hunt: The Vanishing Tally       | HashMap `insert` clobber vs `entry()`    |
| ★ 76 | Bug Hunt: The Silent Zero           | `unwrap_or(0)` swallows errors vs `?`    |
| ★ 77 | Bug Hunt: The Missing Critical Beds | inverted `filter` predicate + `fold`     |
| ★ 78 | Bug Hunt: The Overlooked Rush       | slice-window off-by-one: `..` vs `..=`   |
| ★ 79 | Bug Hunt: The Double-Booked Borrow  | RefCell `borrow` held across `borrow_mut`|
| ★ 80 | Bug Hunt: The Half-Heard Clock-Out  | drain mpsc with `while let`, not `recv`  |

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

**Async (c50-c52)** — first `async fn` + `.await` against the clinic scheduler, then `tokio::spawn` to book two patients at once, then `mpsc` channels from nurses to the nurses' station.

**CLI Project (c53-c54)** — `clap` arg parsing for a medication order, then the capstone: a ward care-log CLI.

### RUST GENERAL HOSPITAL (c55–c74)

Each lesson is one small, real job that hospital software does — no story arc. Mai, Linh, and Trang are the nurses, and the ⚡ warmups use the hospital's medicine-delivery robot. Each ⚡ warmup uses c01–c54 tools only; each ★ hands you broken code to debug.

**Ward Equipment (c55–c58)** — count the robot's ways up a staircase (⚡ Fibonacci), plan its delivery route (recursive `Box`), rack the bedside monitor's sensors (`Box<dyn Trait>` vtables), then debug the infusion pump whose `Deref` serves the factory default instead of the programmed rate.

**Safe Shutdown (c59–c61)** — pair two hallway legs for the robot's last trip (⚡ Two Sum), make every IV pump stop when its session ends (`Drop`/RAII, LIFO), then break the Rc cycle that keeps a bed alarm ringing for a discharged patient (`Weak<T>`).

**Shared Care (c62–c64)** — retrace the robot's path in place (⚡ two-pointer), run the shared crash-cart defibrillator (`Cell` `replace`/`take` through `&self`), then stop the patient chart from crashing when a note arrives mid-review (`try_borrow_mut`).

**Night Shift (c65–c68)** — catch a medication delivered twice (⚡ HashSet), let three nurses double-check one supply list across threads (`Arc`), find where ten nurses' fluid entries vanish (`Arc<Mutex<T>>` — the threaded mirror of c44's `Rc<RefCell<T>>`), and run the bed board (`RwLock`).

**Pharmacy (c69–c74)** — persistent stock records on `sled`: store them, look them up, encode a `Stock` struct with serde, take stock, build the reorder list — and the CAPSTONE imports **your own c71 codec** to load the supplier's delivery CSV. `PHARMACY ONLINE`.

### Bug Hunt (c75–c80)

Side jobs around the hospital — the back-office code is broken and you debug it. No new concepts, pure diagnosis reps on c01–c54 material: a HashMap clobber, a swallowed parse error, an inverted filter, a slice off-by-one, a RefCell double borrow, a half-drained channel. Each ships compiling-but-wrong with a `// BUG:` symptom note; the example file holds the corrected reference.

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
                                                                                                              c53 (clap) → c54 (ward CLI capstone)
                                                                                                                                        │
                                                                                                                                        ▼
   [Ward Equipment]      ⚡c55 (stairs) → c56 (route) → c57 (monitor) → ★c58 (pump)
                                                                           │
                                                                           ▼
   [Safe Shutdown]       ⚡c59 (last trip) → c60 (pump stop) → ★c61 (bed alarm)
                                                                    │
                                                                    ▼
   [Shared Care]         ⚡c62 (retrace) → c63 (crash cart) → ★c64 (chart review)
                                                                  │
                                                                  ▼
   [Night Shift]         ⚡c65 (double dose) → c66 (supply check) → ★c67 (fluids) → c68 (bed board)
                                                                                  │
                                                                                  ▼
   [Pharmacy]            c69 (store) → c70 (lookup) → c71 (codec) → c72 (stock take) → c73 (reorder)
                                                                                        │
                                                                                        ▼
                                                        c74 (CAPSTONE — imports YOUR c71 codec)
                                                                                        │
                                                                                        ▼
   [Bug Hunt]            ★c75 → ★c76 → ★c77 → ★c78 → ★c79 → ★c80  (hospital side jobs, any order)
```

---

## Post-80 Track

The 80-lesson curriculum covers the core language, practical application, and debugging reps. Once you clear c80, good next topics:

1. **Procedural and derive macros** — `macro_rules!`, `#[derive(...)]` custom macros.
2. **Advanced async** — `select!`, `tokio::sync::RwLock`, cancellation, backpressure.
3. **Web server** — `axum` or `actix-web` with the ward care log as an API.
4. **Relational / SQL databases** — `sqlx` or `SeaORM` over SQLite/Postgres (you've done embedded key-value with `sled`; next is querying a SQL engine).
5. **Testing patterns** — property-based testing, mocking, integration test organization.

---

## One Rule

**One new concept per lesson.** If a lesson introduces more than one new thing, it's a bug — flag it and we'll split it.
