# Rust Concept: Traits
#
# N/A in Python (not directly). Rust uses traits to define shared behavior:
#   trait Describable { fn describe(&self) -> String; }
#   impl Describable for Item { ... }
#
# Python uses duck typing — if it has a .describe() method, it works.
# No explicit "implements" declaration needed.
# The closest Python equivalent is Abstract Base Classes (ABCs):
#   from abc import ABC, abstractmethod
#   class Describable(ABC):
#       @abstractmethod
#       def describe(self) -> str: ...
#
# Key difference: Rust traits are checked at compile time.
# Python duck typing is checked at runtime (crashes if method missing).
