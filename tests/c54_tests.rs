#[path = "../src/bin/c54_exercise.rs"]
#[allow(dead_code)]
mod c54_exercise;

use c54_exercise::Salon;

#[test]
fn book_and_list() {
    let mut salon = Salon::new();
    salon.book("Mai", "Gel Manicure", 4500);
    salon.book("Linh", "Pedicure", 3500);
    assert_eq!(salon.list().len(), 2);
    assert_eq!(salon.list()[0].0, "Mai");
    assert_eq!(salon.list()[1].2, 3500);
}

#[test]
fn revenue_by_technician() {
    let mut salon = Salon::new();
    salon.book("Mai", "Gel Manicure", 4500);
    salon.book("Linh", "Pedicure", 3500);
    salon.book("Mai", "Acrylic Fill", 3000);

    let rev = salon.revenue_by_tech();
    assert_eq!(rev.get("Mai"), Some(&7500));
    assert_eq!(rev.get("Linh"), Some(&3500));
}

#[test]
fn empty_salon() {
    let salon = Salon::new();
    assert!(salon.list().is_empty());
    assert!(salon.revenue_by_tech().is_empty());
}
