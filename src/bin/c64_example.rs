// RefCell also gives interior mutability, but for non-Copy data, by moving the borrow check from
// compile time to RUNTIME: borrow()/borrow_mut() track active borrows and PANIC on a violation.
// Coming from C: like a runtime assert that catches aliasing a mutable thing — the safety the
// compiler usually proves statically, enforced dynamically instead. try_borrow_mut avoids the panic.
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec!["Mai"]);

    {
        let mut first = cell.borrow_mut();
        first.push("Linh");
        // A second borrow *while `first` is alive* would PANIC at runtime:
        //   let second = cell.borrow_mut(); // thread panics: BorrowMutError
        // try_borrow_mut() returns Err instead of panicking:
        println!("second borrow ok? {}", cell.try_borrow_mut().is_ok()); // false
    } // `first` is released here

    println!("now ok? {}", cell.try_borrow_mut().is_ok()); // true
    println!("{:?}", cell.borrow());
}
