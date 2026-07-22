// THE VAULT RUN — Chapter 2: GHOST PROTOCOL — ★ BUG HUNT ★
//
// BUG: You jack out, but the session never dies. The trace daemon and your session
// point at each other, and strong_count says TWO owners when you expected one —
// an Rc cycle. The count never hits zero, the memory never frees, and the daemon
// can still reach you after disconnect. The trace follows you home.
//
// One direction of this handshake must not own the other.
// Find it, fix it: cargo test --test c61_tests
#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

pub struct Session {
    pub id: String,
    pub daemon: RefCell<Option<Rc<Daemon>>>,
}

pub struct Daemon {
    pub target: RefCell<Option<Rc<Session>>>,
}

pub fn open_session() -> (Rc<Session>, Rc<Daemon>) {
    let session = Rc::new(Session {
        id: "ghost-run-7".to_string(),
        daemon: RefCell::new(None),
    });
    let daemon = Rc::new(Daemon {
        target: RefCell::new(None),
    });
    *session.daemon.borrow_mut() = Some(Rc::clone(&daemon));
    *daemon.target.borrow_mut() = Some(Rc::clone(&session));
    (session, daemon)
}

pub fn link_counts() -> (usize, usize) {
    // How many strong and weak owners does the session have while the daemon watches?
    let (session, _daemon) = open_session();
    (Rc::strong_count(&session), Rc::weak_count(&session))
}

pub fn trace_lost() -> bool {
    // Jack out. Does the daemon lose you?
    let (session, daemon) = open_session();
    drop(session);
    let watching = daemon.target.borrow();
    watching.is_none()
}

fn main() {
    println!("[session] (strong, weak) = {:?} — want (1, 1)", link_counts());
    println!("[session] trace lost after jack-out? {} — want true", trace_lost());
    println!("══ when the trace loses you, CHAPTER 2: GHOST PROTOCOL is complete ══");
}
