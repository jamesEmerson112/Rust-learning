// A type can't contain itself by value — that would be infinitely large. Box<T> is a pointer
// to the heap with a known, fixed size, which is what makes recursive types (lists, trees) work.
// Coming from C: it's exactly why a linked-list node holds a `next` POINTER, not the next node
// inline. Box is that pointer — but it owns what it points to and frees it automatically.
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
