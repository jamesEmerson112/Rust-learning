// This is just a singly linked list. In C you'd write:
//
//     struct Node { char *name; struct Node *next; };   /* next is a POINTER */
//
// `next` can't be a `struct Node` held by value — a node containing a whole node
// containing a whole node would be infinitely large, so C makes you use a pointer.
// Rust has the same rule: `Hop(String, Route)` won't compile, because Route would
// contain itself. Box<Route> is that pointer — fixed size (8 bytes), points to the
// heap. The difference from C: Box OWNS what it points to, so dropping the head
// frees the entire chain automatically. No free() loop.
//
// THE VAULT RUN: your intrusion route into Aegis-9 is a chain of compromised nodes.
// Each hop knows only the next hop — a cons-list, like nature intended.
enum Route {
    Hop(String, Box<Route>),
    Exit,
}

fn build_route(nodes: &[&str]) -> Route {
    let mut route = Route::Exit;
    for node in nodes.iter().rev() {
        route = Route::Hop(node.to_string(), Box::new(route));
    }
    route
}

fn hop_count(route: &Route) -> usize {
    match route {
        Route::Hop(_, rest) => 1 + hop_count(rest),
        Route::Exit => 0,
    }
}

fn last_node(route: &Route) -> Option<String> {
    match route {
        Route::Hop(name, rest) => match rest.as_ref() {
            Route::Exit => Some(name.clone()),
            _ => last_node(rest),
        },
        Route::Exit => None,
    }
}

fn main() {
    let route = build_route(&["gateway", "relay-7", "aegis-core"]);
    println!("[route] {} hops plotted", hop_count(&route));
    if let Some(door) = last_node(&route) {
        println!("[route] final hop before the vault: {door}");
    }
}
