use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let book: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    let station_a = Rc::clone(&book);
    let station_b = Rc::clone(&book);

    station_a.borrow_mut().push("Mai - 10:00 Gel Manicure".to_string());
    station_b.borrow_mut().push("Linh - 10:30 Pedicure".to_string());

    println!("Shared book has {} appointments", book.borrow().len());
    for appt in book.borrow().iter() {
        println!("  {appt}");
    }
}
