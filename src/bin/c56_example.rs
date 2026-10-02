// This is just a singly linked list. In C you'd write:
//
//     struct Node { char *name; struct Node *next; };   /* next is a POINTER */
//
// `next` can't be a `struct Node` held by value — a node containing a whole node
// containing a whole node would be infinitely large, so C makes you use a pointer.
// Rust has the same rule: `Stop(String, Route)` won't compile, because Route would
// contain itself. Box<Route> is that pointer — fixed size (8 bytes), points to the
// heap. The difference from C: Box OWNS what it points to, so dropping the head
// frees the entire chain automatically. No free() loop.
//
// RUST GENERAL HOSPITAL: the delivery robot's route is a chain of stops. Each stop knows
// only the next one, and the route ends when the robot is back at its dock.
enum Route {
    Stop(String, Box<Route>),
    Dock,
}

fn build_route(stops: &[&str]) -> Route {
    let mut route = Route::Dock;
    for stop in stops.iter().rev() {
        route = Route::Stop(stop.to_string(), Box::new(route));
    }
    route
}

fn stop_count(route: &Route) -> usize {
    match route {
        Route::Stop(_, rest) => 1 + stop_count(rest),
        Route::Dock => 0,
    }
}

fn last_stop(route: &Route) -> Option<String> {
    match route {
        Route::Stop(name, rest) => match rest.as_ref() {
            Route::Dock => Some(name.clone()),
            _ => last_stop(rest),
        },
        Route::Dock => None,
    }
}

fn main() {
    let route = build_route(&["pharmacy", "ward-3", "icu"]);
    println!("[robot] {} stops planned", stop_count(&route));
    if let Some(stop) = last_stop(&route) {
        println!("[robot] last stop before the dock: {stop}");
    }
}
