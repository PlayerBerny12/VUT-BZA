//! Pseudo random number generator module

use super::RNG;
use crate::manager_rng;
use std::num::Wrapping;

// Placeholder for pseudo RNG
#[derive(Debug)]
pub struct Xorshift32RNG {
    state: u32,
}

// Placeholder for pseudo RNG
impl Xorshift32RNG {}

// Placeholder for pseudo RNG
impl manager_rng::RNG for Xorshift32RNG {
    fn new(seed: &u32) -> Xorshift32RNG {
        Xorshift32RNG { state: *seed }
    }

    fn next(&mut self) -> u32 {
        let mut x: Wrapping<u32> = Wrapping(self.state);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x.0;

        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshitf32() {
        let seed: u32 = 28664236;
        let mut rng = Xorshift32RNG::new(&seed);

        assert!(rng.next() == 496392940);
        assert!(rng.next() == 264077481);
        assert!(rng.next() == 1253511389);
    }
}
