//! Pseudo random number generator module

use crate::manager_rng;

use super::RNG;

#[derive(Debug)]
pub struct PseudoRNG {
    seed: u32
}

impl PseudoRNG {
    
}

impl manager_rng::RNG for PseudoRNG {
    fn new(seed: &u32) -> PseudoRNG {
        PseudoRNG{seed: *seed}
    }

    // PLACEHOLDER for get_next_rand function
    fn next(&self) -> u32 {
        self.seed
    }
}