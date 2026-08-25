// THE VAULT RUN — Chapter 1: LOADOUT
// The job is on. Target: the Aegis-9 corporate vault. Step one — their rolling access
// code is Fibonacci-derived. Build the keygen. (Warmup: no new Rust concepts.)

pub fn access_code(n: u32) -> u64 {
    // TODO: Return the n-th code in the keystream. access_code(0) = 0, access_code(1) = 1.
    // Hint: keep a rolling pair and swap n times: (a, b) = (b, a + b).
    let (mut a, mut b) = (0, 0);

    for _ in 0..n {
        (a, b) = (b, a+b);
    }

    a
}

pub fn keystream(len: usize) -> Vec<u64> {
    // TODO: Return the first `len` codes in order.
    // Hint: (0..len).map(|n| access_code(n as u32)).collect()
    (0..len).map(|n| access_code(n as u32)).collect()
}

fn main() {
    println!("[keygen] access_code(10) = {} (want 55)", access_code(10));
    println!("[keygen] keystream(8) = {:?}", keystream(8));
}
