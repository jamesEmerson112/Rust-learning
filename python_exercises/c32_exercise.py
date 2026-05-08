# Rust Concept: Modules & Visibility
#
# N/A in Python (it's trivial). In Rust, modules control visibility:
#   pub struct Counter { count: u32 }     // count is private
#   pub fn new() -> Self { ... }          // new() is public
#   pub fn value(&self) -> u32 { ... }    // value() is public
#
# In Python, every file IS a module. There's no pub keyword.
# Convention: prefix with _ for "private" (e.g., _internal_fn),
# but nothing enforces it — Python trusts the developer.
#
# Rust enforces visibility at compile time. Python is "we're all adults here."
