// Rc counts owners and frees at zero — but two Rcs pointing at each other form a cycle whose
// count never reaches zero: a leak. Weak<T> is a non-owning reference that doesn't bump the
// count, breaking the cycle. Coming from C: a manual refcount where a child's back-pointer to
// its parent is deliberately "weak" so it can't keep the parent alive forever.
//
// RUST GENERAL HOSPITAL: a bed alarm watches its patient. If the alarm holds a STRONG grip,
// discharging the patient never frees their record and the alarm keeps ringing for an empty
// bed. The alarm must watch through a Weak: after discharge, upgrade() comes back None.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Patient {
    name: String,
    alarm: RefCell<Option<Rc<BedAlarm>>>,
}

struct BedAlarm {
    patient: RefCell<Option<Weak<Patient>>>, // weak grip — can't keep the patient alive
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
    *alarm.patient.borrow_mut() = Some(Rc::downgrade(&patient)); // downgrade, not clone

    println!(
        "[{}] strong = {}, weak = {}",
        patient.name,
        Rc::strong_count(&patient),
        Rc::weak_count(&patient)
    );

    drop(patient); // discharge — the only strong owner is gone

    let watching = alarm.patient.borrow();
    match watching.as_ref().and_then(|w| w.upgrade()) {
        Some(p) => println!("[alarm] still ringing for {}", p.name),
        None => println!("[alarm] patient discharged. alarm silent."),
    }
}
