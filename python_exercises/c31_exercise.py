# Rust Concept: Rc<T> — Reference Counting
#
# N/A in Python (it's the default). Rust's Rc<T> enables shared ownership:
#   let a = Rc::new(42);
#   let b = Rc::clone(&a);  // both a and b point to the same 42
#   Rc::strong_count(&a)    // returns 2
#
# Python uses reference counting for ALL objects by default:
#   import sys
#   a = [1, 2, 3]
#   b = a                   # both point to the same list
#   sys.getrefcount(a)      # returns 3 (a, b, + getrefcount's own ref)
#
# In Rust, Rc<T> is opt-in and single-threaded. Arc<T> is the thread-safe version.
# In Python, refcounting is always on, plus a GC handles reference cycles.
