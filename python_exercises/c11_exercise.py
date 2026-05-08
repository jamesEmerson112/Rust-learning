# Rust Concept: Enums with Payloads
#
# N/A in Python (not directly). Rust enums carry data per variant:
#   enum Command {
#       Add(i32, i32),
#       Sub(i32, i32),
#       Quit,
#   }
#
# Python's enum.Enum only supports simple values (like C enums).
# To carry per-variant data, you'd use dataclasses or tagged unions:
#   @dataclass
#   class Add: a: int; b: int
#   @dataclass
#   class Sub: a: int; b: int
#   Command = Add | Sub | None   # Python 3.10+ union type
#
# Rust's match on enums is exhaustive (compiler checks all variants).
# Python's match/case (3.10+) is not exhaustive by default.
