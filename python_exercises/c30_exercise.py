# Rust Concept: Box<T> — Heap Allocation
#
# N/A in Python. In Rust, values live on the stack by default.
# Box<T> explicitly moves a value to the heap:
#   let x = Box::new(42);  // 42 is now on the heap
#
# In Python, EVERYTHING is heap-allocated. Every int, str, list, dict
# is an object on the heap. There is no stack-vs-heap choice.
#
# Box<T> exists in Rust because stack allocation is the default.
# Python doesn't need Box because heap allocation is the ONLY option.
