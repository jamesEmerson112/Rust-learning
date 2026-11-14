//! Limits how fast commands reach a robot's motor controller with a token bucket.
//!
//! APIs:
//! - `TokenBucket`: the bucket's size, refill rate and current fill
//! - `TokenBucket::new`: a full bucket
//! - `TokenBucket::try_acquire`: spend one token if one is available
//! - `TokenBucket::tokens`: the whole tokens available at a moment

use std::cmp::{max, min};

/// Description: The number of millitokens in one token. The bucket counts thousandths of a token
/// so that a slow refill rate never loses the fractions between calls.
const MILLI: u64 = 1_000;

/// Description: A token bucket. Each command spends one token, and tokens flow back at a fixed
/// rate up to the bucket's size. This allows short bursts but caps the average rate.
/// Fields:
/// - `capacity`: the most tokens the bucket holds, which is the largest burst.
/// - `refill_per_sec`: the tokens added per second.
/// - `millitokens`: the current fill, in thousandths of a token.
/// - `last_ms`: the latest time the bucket has seen, in milliseconds.
pub struct TokenBucket {
    capacity:       u64,
    refill_per_sec: u64,
    millitokens:    u64,
    last_ms:        u64,
}

/// Description: Sends a burst of commands and a few later ones through a three-token bucket that
/// refills two tokens a second, printing which go through.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut bucket = TokenBucket::new(3, 2, 0);

    for now_ms in [0, 0, 0, 0, 400, 500, 1_000] {
        // Compressed:
        // println!("t={now_ms}ms sent={}", bucket.try_acquire(now_ms));
        let sent = bucket.try_acquire(now_ms);
        println!("t={now_ms}ms sent={sent}");
    }
}

impl TokenBucket {
    /// Description: Builds a full bucket.
    /// Inputs:
    /// - `capacity`: the most tokens the bucket holds.
    /// - `refill_per_sec`: the tokens added per second.
    /// - `now_ms`: the current time in milliseconds.
    /// Outputs: the new bucket.
    pub fn new(capacity: u64, refill_per_sec: u64, now_ms: u64) -> Self {
        // Compressed:
        // TokenBucket { capacity, refill_per_sec, millitokens: capacity * MILLI, last_ms: now_ms }
        let full_millitokens = capacity * MILLI;
        TokenBucket {
            capacity,
            refill_per_sec,
            millitokens: full_millitokens,
            last_ms:     now_ms,
        }
    }

    /// Description: Spends one token if the bucket has one.
    /// Inputs:
    /// - `now_ms`: the current time in milliseconds.
    /// Outputs: `true` when the command may go through, or `false` when it must wait.
    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        self.refill(now_ms);

        if self.millitokens >= MILLI {
            self.millitokens -= MILLI;
            true
        } else {
            false
        }
    }

    /// Description: Returns how many whole tokens are available.
    /// Inputs:
    /// - `now_ms`: the current time in milliseconds.
    /// Outputs: the number of commands that could go through right now.
    pub fn tokens(&mut self, now_ms: u64) -> u64 {
        self.refill(now_ms);
        self.millitokens / MILLI
    }

    /// Description: Adds the tokens that flowed in since the latest time seen. One token per
    /// second is one millitoken per millisecond, so the refill is the elapsed milliseconds times
    /// the rate.
    /// Inputs:
    /// - `now_ms`: the current time in milliseconds.
    /// Outputs: none
    /// Constraints: a time earlier than one already seen adds nothing, so a clock that jumps
    /// backwards cannot underflow the subtraction or create extra tokens.
    fn refill(&mut self, now_ms: u64) {
        // Compressed:
        // let elapsed_ms = now_ms.saturating_sub(self.last_ms);
        // let added = elapsed_ms.saturating_mul(self.refill_per_sec);
        // self.millitokens = self.millitokens.saturating_add(added).min(self.capacity * MILLI);
        // self.last_ms = self.last_ms.max(now_ms);
        let elapsed_ms       = now_ms.saturating_sub(self.last_ms);
        let added            = elapsed_ms.saturating_mul(self.refill_per_sec);
        let refilled         = self.millitokens.saturating_add(added);
        let full_millitokens = self.capacity * MILLI;

        self.millitokens = min(refilled, full_millitokens);
        self.last_ms     = max(self.last_ms, now_ms);
    }
}
