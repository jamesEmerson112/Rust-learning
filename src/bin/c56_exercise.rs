// RUST GENERAL HOSPITAL — Ward Equipment
// Plan the delivery robot's route: a chain of stops, each one holding the REST of the
// route behind a Box. Without Box this enum would be infinitely large — the Box stores
// the tail behind a fixed-size heap pointer so the compiler can size it.
pub enum Route {
    Stop(String, Box<Route>),
    Dock,
}

pub fn build_route(stops: &[&str]) -> Route {
    // TODO: Nest the stops into a Route ending in Dock, first stop outermost.
    // Hint: start from Route::Dock and fold from the BACK of the slice
    // (`stops.iter().rev()`), wrapping each name around what you have so far.

    let mut route = Route::Dock;
    for node in stops.iter().rev() {
        // create a new hop
        // linked list one item -> next
        route = Route::Stop(node.to_string(), Box::new(route));
    }
    Route::Dock
}

pub fn stop_count(route: &Route) -> usize {
    // TODO: Recurse — 1 + the stops in the rest, or 0 for Dock.
    match route {
        Route::Stop(_, rest) => 1 + stop_count(rest),
        Route::Dock => 0
    }
}

pub fn last_stop(route: &Route) -> Option<String> {
    // TODO: Return the DEEPEST stop name — the one right before Dock — or None
    // for an empty route. Hint: peek at the tail with `rest.as_ref()`.
    match route {
        Route::Stop(item_name, rest) => match rest.as_ref() {
            Route::Dock => Some(item_name.clone()),
            _ => last_stop(rest)
        },
        Route::Dock => None,
    }
}

fn main() {
    let route = build_route(&["pharmacy", "ward-3", "icu"]);
    println!("[robot] {} stops planned (want 3)", stop_count(&route));
    println!("[robot] last stop before the dock: {:?} (want Some(\"icu\"))", last_stop(&route));
}
