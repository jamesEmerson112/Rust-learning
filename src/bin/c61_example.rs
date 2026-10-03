// Rc counts the owners of a value and frees the value when the count reaches zero. If two
// Rc values point at each other, they form a cycle. Each one keeps the other's count above
// zero, so neither is ever freed, and the memory leaks. Weak<T> is a reference that does not
// own the value and does not add to its strong count, so it breaks the cycle.
// Coming from C: think of a manually reference-counted tree where a child's pointer back to
// its parent does not increment the parent's count, so the child cannot keep the parent alive.
//
// RUST GENERAL HOSPITAL: a bed alarm watches its patient. If the alarm holds a strong Rc to
// the patient, discharging the patient never frees the record, and the alarm keeps ringing
// for an empty bed. The alarm watches through a Weak instead, so after discharge, upgrade()
// returns None.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Patient {
    name: String,
    alarm: RefCell<Option<Rc<BedAlarm>>>,
}

struct BedAlarm {
    patient: RefCell<Option<Weak<Patient>>>, // a Weak does not keep the patient alive
}

fn main() {
    let patient = Rc::new(Patient {
        name: "Mr. Hung".to_string(),
        alarm: RefCell::new(None),
    });
    let alarm = Rc::new(BedAlarm {
        patient: RefCell::new(None),
    });

    *patient.alarm.borrow_mut() = Some(Rc::clone(&alarm));
    *alarm.patient.borrow_mut() = Some(Rc::downgrade(&patient)); // makes a Weak, not a clone

    println!(
        "[{}] strong = {}, weak = {}",
        patient.name,
        Rc::strong_count(&patient),
        Rc::weak_count(&patient)
    );

    drop(patient); // discharge the patient, which drops the only strong owner

    let watching = alarm.patient.borrow();
    match watching.as_ref().and_then(|w| w.upgrade()) {
        Some(p) => println!("[alarm] still ringing for {}", p.name),
        None => println!("[alarm] patient discharged. alarm silent."),
    }
}
