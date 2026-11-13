//! Tests for the motor-command token bucket in `src/bin/d11_rate_limiter.rs`.
//!
//! Contents:
//! - burst test: a full bucket allows up to its capacity at once
//! - refill tests: tokens flow back, never overflow, and keep fractions
//! - clock test: a clock that jumps backwards adds nothing

#[path = "../src/bin/d11_rate_limiter.rs"]
#[allow(dead_code)]
mod d11_rate_limiter;

use d11_rate_limiter::TokenBucket;

/// Description: A full three-token bucket lets three commands through at once and stops the
/// fourth.
/// Inputs: none
/// Outputs: none
#[test]
fn a_full_bucket_allows_a_burst_up_to_capacity() {
    let mut bucket = TokenBucket::new(3, 1, 0);
    assert!(bucket.try_acquire(0));
    assert!(bucket.try_acquire(0));
    assert!(bucket.try_acquire(0));
    assert!(!bucket.try_acquire(0));
}

/// Description: At one token per second, the next token arrives exactly 1,000 milliseconds later.
/// Inputs: none
/// Outputs: none
#[test]
fn tokens_flow_back_with_time() {
    let mut bucket = TokenBucket::new(1, 1, 0);
    assert!(bucket.try_acquire(0));
    assert!(!bucket.try_acquire(999));
    assert!(bucket.try_acquire(1_000));
    assert!(!bucket.try_acquire(1_000));
}

/// Description: A long idle stretch refills the bucket only up to its capacity.
/// Inputs: none
/// Outputs: none
#[test]
fn the_refill_never_overflows_the_bucket() {
    let mut bucket = TokenBucket::new(2, 10, 0);
    bucket.try_acquire(0);
    bucket.try_acquire(0);
    assert_eq!(bucket.tokens(10_000), 2);
}

/// Description: At three tokens per second a token takes 333.3 milliseconds, so 333 is not quite
/// enough and 334 is.
/// Inputs: none
/// Outputs: none
#[test]
fn fractions_of_a_token_are_kept() {
    let mut bucket = TokenBucket::new(1, 3, 0);
    assert!(bucket.try_acquire(0));
    assert!(!bucket.try_acquire(333));
    assert!(bucket.try_acquire(334));
}

/// Description: A time earlier than one already seen adds no tokens and does not panic, and the
/// refill resumes from the latest time seen.
/// Inputs: none
/// Outputs: none
#[test]
fn a_clock_that_jumps_backwards_adds_nothing() {
    let mut bucket = TokenBucket::new(1, 1, 5_000);
    assert!(bucket.try_acquire(5_000));
    assert!(!bucket.try_acquire(1_000));
    assert_eq!(bucket.tokens(1_000), 0);
    assert!(bucket.try_acquire(6_000));
}
