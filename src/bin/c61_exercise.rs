// RUST GENERAL HOSPITAL — Safe Shutdown — ★ BUG HUNT ★
//
// BUG: Mr. Hung went home, but his bed alarm keeps ringing for an empty bed. The patient
// record and the bed alarm point at each other through Rc, which forms a cycle.
// strong_count reports two owners for the patient when you expected one. Because the count
// never reaches zero, the record is never freed, and the alarm can still reach a patient who
// was discharged.
//
// Find the bug and fix it, then run: cargo test --test c61_tests
#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

pub struct Patient {
    pub name: String,
    pub alarm: RefCell<Option<Rc<BedAlarm>>>,
}

pub struct BedAlarm {
    pub patient: RefCell<Option<Rc<Patient>>>,
}

pub fn admit() -> (Rc<Patient>, Rc<BedAlarm>) {
    let patient = Rc::new(Patient {
        name: "Mr. Hung".to_string(),
        alarm: RefCell::new(None),
    });
    let alarm = Rc::new(BedAlarm {
        patient: RefCell::new(None),
    });
    *patient.alarm.borrow_mut() = Some(Rc::clone(&alarm));
    *alarm.patient.borrow_mut() = Some(Rc::clone(&patient));
    (patient, alarm)
}

pub fn link_counts() -> (usize, usize) {
    // How many strong and weak owners does the patient have while the alarm watches?
    let (patient, _alarm) = admit();
    (Rc::strong_count(&patient), Rc::weak_count(&patient))
}

pub fn alarm_silent_after_discharge() -> bool {
    // Discharge the patient. Does the alarm let go?
    let (patient, alarm) = admit();
    drop(patient);
    let watching = alarm.patient.borrow();
    watching.is_none()
}

fn main() {
    println!("[patient] (strong, weak) = {:?} — want (1, 1)", link_counts());
    println!("[alarm] silent after discharge? {} — want true", alarm_silent_after_discharge());
    println!("══ when the alarm goes quiet, Safe Shutdown is complete ══");
}
