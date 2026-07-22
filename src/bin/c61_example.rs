// Rc counts owners and frees at zero — but two Rcs pointing at each other form a cycle whose
// count never reaches zero: a leak. Weak<T> is a non-owning reference that doesn't bump the
// count, breaking the cycle. Coming from C: a manual refcount where a child's back-pointer to
// its parent is deliberately "weak" so it can't keep the parent alive forever.
//
// THE VAULT RUN: Aegis-9's trace daemon watches your session. If the daemon holds a STRONG
// grip, jacking out never frees the session — the trace follows you home. The daemon must
// watch through a Weak: when you disconnect, upgrade() comes back None. Ghost protocol.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Session {
    id: String,
    daemon: RefCell<Option<Rc<Daemon>>>,
}

struct Daemon {
    target: RefCell<Option<Weak<Session>>>, // weak grip — can't keep the session alive
}

fn main() {
    let session = Rc::new(Session {
        id: "ghost-run-7".to_string(),
        daemon: RefCell::new(None),
    });
    let daemon = Rc::new(Daemon {
        target: RefCell::new(None),
    });

    *session.daemon.borrow_mut() = Some(Rc::clone(&daemon));
    *daemon.target.borrow_mut() = Some(Rc::downgrade(&session)); // downgrade, not clone

    println!(
        "[session {}] strong = {}, weak = {}",
        session.id,
        Rc::strong_count(&session),
        Rc::weak_count(&session)
    );

    drop(session); // jack out — the only strong owner is gone

    let watching = daemon.target.borrow();
    match watching.as_ref().and_then(|w| w.upgrade()) {
        Some(s) => println!("[daemon] still tracing {}", s.id),
        None => println!("[daemon] target lost. ghost protocol holds."),
    }
}
