# Rust Concept: Lifetimes ('a)
#
# N/A in Python. Rust lifetimes tell the compiler how long references are valid:
#   fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
#
# This means: "the returned reference lives as long as the shorter-lived input."
# The compiler uses this to prevent dangling references at compile time.
#
# Python has garbage collection — objects live as long as something references them.
# No dangling pointers, no lifetime annotations, no compile-time checking.
# The tradeoff: Python pays for GC overhead at runtime. Rust pays zero.
