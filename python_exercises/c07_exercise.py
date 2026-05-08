# Rust Concept: Ownership & Borrowing
#
# N/A in Python. Python uses reference counting + garbage collection.
# There is no "ownership" — every variable is a reference to a heap object.
# You never need to think about who "owns" a value or whether a borrow is valid.
#
# In Rust:  fn shout(message: &str) -> String  — borrows the input, returns owned String
# In Python: strings are immutable objects. No distinction between &str and String.
