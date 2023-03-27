//! Manager module for random number generator modules

use self::prng::PseudoRNG;

mod prng;

// Generic trait for all random number generators
pub trait RNG {
    fn new(seed: &u32) -> Self where Self: Sized;
    fn next(&self) -> u32;
}

// Random number generator manager structure
pub struct RandomNumberGeneratorManager {
    pub generator: Box<dyn RNG>
}

// Impl for random number generator manager structure
impl RandomNumberGeneratorManager {
    pub fn new_PseudoRNG(seed: &u32) -> Self {
        RandomNumberGeneratorManager { generator: Box::new(PseudoRNG::new(seed)) }
    }

    pub fn new_seed(&mut self, seed: &u32) {
        self.generator = Box::new(PseudoRNG::new(seed));
    }
}