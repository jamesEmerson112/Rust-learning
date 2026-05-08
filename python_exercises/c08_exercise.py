# Rust Concept: String Slices (&str)
#
# N/A in Python. Python strings are immutable objects — slicing creates a new string.
# In Rust, &str is a borrowed view into existing string data (no allocation).
# Python's str[1:5] copies bytes into a new str object every time.
#
# In Rust:  &str is a pointer + length (zero-copy view)
# In Python: str is always a full object on the heap
