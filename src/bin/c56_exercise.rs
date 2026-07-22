// THE VAULT RUN — Chapter 1: LOADOUT
// Plot the intrusion route: a chain of compromised nodes, each one holding the REST
// of the route behind a Box. Without Box this enum would be infinitely large —
// the Box stores the tail behind a fixed-size heap pointer so the compiler can size it.
pub enum Route {
    Hop(String, Box<Route>),
    Exit,
}

pub fn build_route(nodes: &[&str]) -> Route {
    // TODO: Nest the nodes into a Route ending in Exit, first node outermost.
    // Hint: start from Route::Exit and fold from the BACK of the slice
    // (`nodes.iter().rev()`), wrapping each name around what you have so far.
    let _ = nodes;
    Route::Exit
}

pub fn hop_count(route: &Route) -> usize {
    // TODO: Recurse — 1 + the hops in the rest, or 0 for Exit.
    let _ = route;
    0
}

pub fn last_node(route: &Route) -> Option<String> {
    // TODO: Return the DEEPEST hop name — the node right before Exit — or None
    // for an empty route. Hint: peek at the tail with `rest.as_ref()`.
    let _ = route;
    None
}

fn main() {
    let route = build_route(&["gateway", "relay-7", "aegis-core"]);
    println!("[route] {} hops plotted (want 3)", hop_count(&route));
    println!("[route] vault doorstep: {:?} (want Some(\"aegis-core\"))", last_node(&route));
}
