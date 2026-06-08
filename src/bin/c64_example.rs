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
