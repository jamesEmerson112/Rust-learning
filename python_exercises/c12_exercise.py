# Rust Concept: Option<T>
#
# N/A in Python. Rust wraps possibly-absent values in Option<T>:
#   Some(42)  — value exists
#   None      — no value
#
# Python uses None directly — no wrapper type. A function returns int or None:
#   def safe_get(items, index):
#       if index < len(items): return items[index]
#       return None
#
# The difference: Rust's type system FORCES you to handle None.
# Python's None can sneak through and crash at runtime (AttributeError).
# Rust prevents NullPointerException at compile time. Python doesn't.
