# Rust Concept: impl Blocks & &self Methods
#
# N/A in Python. In Python, methods live inside the class definition.
# In Rust, data (struct) and behavior (impl) are separate:
#   struct Rectangle { width: u32, height: u32 }
#   impl Rectangle { fn area(&self) -> u32 { ... } }
#
# Python equivalent is just a normal class:
#   class Rectangle:
#       def area(self) -> int: ...
#
# The concept of &self (immutable borrow) vs &mut self (mutable borrow)
# doesn't exist in Python — self is always a mutable reference.
