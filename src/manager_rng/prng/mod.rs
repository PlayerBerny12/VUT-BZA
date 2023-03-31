//! Pseudo random number generator module

use crate::manager_rng;
use super::RNG;

// Placeholder for pseudo RNG
#[derive(Debug)]
pub struct PseudoRNG {
    seed: u32
}

// Placeholder for pseudo RNG
impl PseudoRNG {
    
}

// Placeholder for pseudo RNG
impl manager_rng::RNG for PseudoRNG {
    fn new(seed: &u32) -> PseudoRNG {
        PseudoRNG{seed: *seed}
    }

    fn next(&self) -> u32 {
        self.seed
    }
}