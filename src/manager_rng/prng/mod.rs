//! Pseudo random number generator module

use super::RNG;
use crate::manager_rng;

// Placeholder for pseudo RNG
#[derive(Debug)]
pub struct PseudoRNG {
    seed: u32,
}

// Placeholder for pseudo RNG
impl PseudoRNG {}

// Placeholder for pseudo RNG
impl manager_rng::RNG for PseudoRNG {
    fn new(seed: &u32) -> PseudoRNG {
        PseudoRNG { seed: *seed }
    }

    fn next(&self) -> u32 {
        self.seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let seed: u32 = 1;
        let rng = PseudoRNG::new(&seed);

        assert!(rng.next() == 1);
    }
}
