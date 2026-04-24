# Rust Learning — Roadmap

**33 lessons, ~15 minutes each, one new concept per lesson.**

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
| 30 | `Box<T>` — Heap Allocation            | `Box::new`, recursive types              |
| 31 | `Rc<T>` — Shared Ownership            | `Rc::clone`, reference counting          |
| 32 | Modules and Visibility                | `mod`, `pub`, `use`                      |
| 33 | Capstone: Gradebook                   | everything combined                      |

See [README.md](README.md) Study Plan for the **Learn / Exercise / Done-when** detail on every lesson.

---

## Phases

**Foundations (c01–c06)** — syntax, control flow, primitives. How to write a compilable Rust program.

**Ownership Era (c07–c08)** — the one Rust idea other languages don't have. Everything downstream builds on this.

**Data Modeling (c09–c16)** — structs, enums, `Option`, `HashMap`, strings. Shape your data.

**Error Handling (c17–c19)** — `Result`, then `?`, then `map_err`. Each lesson answers the question the previous one raised.

**Functional Rust (c20–c24)** — closures are the prerequisite for iterator combinators, which unfold one at a time: collect, filter, sum, fold.

**Debugging (c25)** — the `{:?}` format and the Debug trait. Bridges naturally into "what IS a trait?"

**Abstractions (c26–c29)** — traits define contracts, generics consume them, bounds constrain them, lifetimes borrow through them.

**Heap & Sharing (c30–c31)** — `Box` for single owners, `Rc` for many.

**Code Organization (c32)** — `mod` and `pub`. Comes late because you don't need it to learn the language.

**Capstone (c33)** — integrate everything into one working project.

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
                                                                          c17 (Result)
                                                                           │
                                                                           ▼
                                                                          c18 (?)
                                                                           │
                                                                           ▼
                                                                          c19 (map_err)
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
                                                                          c26 (traits) → c27 (generics) → c28 (bounds)
                                                                                                           │
                                                                                                           ▼
                                                                                                          c29 (lifetimes)
                                                                                                           │
                                                                                                           ▼
                                                                                                          c30 (Box) → c31 (Rc)
                                                                                                                       │
                                                                                                                       ▼
                                                                                                                      c32 (modules)
                                                                                                                       │
                                                                                                                       ▼
                                                                                                                      c33 (capstone)
```

Read: each arrow means "the concept on the right depends on the one on the left." You can skip sideways (e.g., start c13 before finishing c12) but don't skip forward across an arrow.

---

## Post-33 Track

The 33-lesson curriculum covers the core language. Once you clear c33, these are the highest-value next concepts — roughly in the order a teacher would tackle them:

1. **Custom Error Enums** — `enum AppError { Io(io::Error), Parse(ParseIntError) }` + `impl Display` + `impl std::error::Error`. Real Rust codebases use these instead of `String` errors. This is the single biggest gap in c01–c33.
2. **Slices `&[T]`** — explicit lesson on slice semantics (DST, bounds, `&vec[..]`). Used implicitly throughout c01–c33.
3. **Struct lifetimes** — `struct Parser<'a> { source: &'a str }`. Extends c29 to the more common real-world case.
4. **`RefCell<T>`** and `Rc<RefCell<T>>` — interior mutability, shared mutable state.
5. **`thiserror` / `anyhow`** — ergonomic error crates once you understand custom error enums.
6. **Custom iterators** — `impl Iterator for MyType`.
7. **Async / `tokio`** — concurrent and asynchronous programming.
8. **Procedural and derive macros** — code generation.

After that: build a small CLI or web service. That forces every concept above into use.

---

## One Rule

**One new concept per lesson.** If a lesson introduces more than one new thing, it's a bug — flag it and we'll split it. (c17 was rewritten for exactly this reason; c21/c22 were split during the 30→33 expansion for the same reason.)
