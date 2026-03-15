# Rust Learning

This repo contains small Rust lessons, each with:
- an `example` binary (complete reference)
- an `exercise` binary (starter stubs with `TODO`s)
- an integration test file for validation

## File layout
- `src/bin/c01_example.rs` ... `src/bin/c20_example.rs`
- `src/bin/c01_exercise.rs` ... `src/bin/c20_exercise.rs`
- `tests/c01_tests.rs` ... `tests/c20_tests.rs`

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

### Lesson 14 — HashMap Entry API and String Normalization
**Learn:** `.entry().or_insert()`/`.or_default()`, `.to_lowercase()`, `trim_matches`
**Exercise:** Count word frequencies in a string, normalising case and punctuation.
**You're done when:** You can use the entry API without looking up the syntax.

---

### Lesson 15 — Result Basics
**Learn:** `Result<T, E>`, `Ok`/`Err`, `.parse::<T>()`, match on Result
**Exercise:** Parse a string as a `u8` age, returning `Err` for invalid input.
**You're done when:** You can return meaningful error messages from a function.

---

### Lesson 16 — The ? Operator and Error Chaining
**Learn:** `?` operator, `.map_err()`, chaining fallible operations
**Exercise:** Parse two strings as numbers and divide them, handling all error cases.
**You're done when:** You can chain `?` calls and return meaningful error strings.

---

### Lesson 17 — Traits
**Learn:** `trait` definition, `impl Trait for Type`, calling trait methods
**Exercise:** Define a `Describable` trait and implement it for an `Item` struct.
**You're done when:** You can define and implement a trait.

---

### Lesson 18 — Generics and Trait Bounds
**Learn:** `<T>`, `T: Ord + Copy`, generic functions
**Exercise:** Write a generic `largest` function that works on any comparable, copyable type.
**You're done when:** You can write a generic function and explain what the trait bounds are doing.

---

### Lesson 19 — Modules and Visibility
**Learn:** `mod`, `pub`, `use`, file-based modules
**Exercise:** Complete a `Counter` struct in a separate module with private fields.
**You're done when:** You can organise code across files and control visibility with `pub`.

---

### Lesson 20 — Capstone: Gradebook
**Learn:** Everything combined: modules + HashMap + Vec + Option + methods
**Exercise:** Complete a `Gradebook` struct in a separate module — add scores and compute averages.
**You're done when:** You can run `cargo test` and all 20 lesson tests pass.

---

### Beyond Chapter 20
The lessons above cover the core language. Once you finish them, good next topics are:
- Lifetimes (`'a`) — when the borrow checker needs extra hints
- Closures and iterators — `.map()`, `.filter()`, `.fold()`
- `Box<T>`, `Rc<T>`, `Arc<T>` — heap allocation and shared ownership
- Async / `tokio` — concurrent programming
- Building a small CLI or web service to apply everything
