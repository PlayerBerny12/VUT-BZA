//! Manager module for random number generator modules

#[cfg(feature = "xorshitf32")]
use self::xorshift::Xorshift32RNG;
#[cfg(feature = "xorshitf32")]
pub mod xorshift;

// Generic trait for all random number generators
pub trait RNG {
    fn new(seed: &u32) -> Self
    where
        Self: Sized;
    fn next(&mut self) -> u32;
}

// Random number generator manager structure
pub struct RandomNumberGeneratorManager {
    seed: u32,
    pub generator: Box<dyn RNG>,
    min: u32,
    max: u32,
    entropy: u32,
}

// Impl for random number generator manager structure
impl RandomNumberGeneratorManager {
    #[cfg(feature = "xorshitf32")]
    pub fn new_PseudoRNG(seed: &u32, min: u32, max: u32) -> Self {
        RandomNumberGeneratorManager {
            seed: *seed,
            generator: Box::new(Xorshift32RNG::new(seed)),
            min: min,
            max: max,
            entropy: 0,
        }
    }

    pub fn new_seed(&mut self, seed: &u32) {
        self.generator = Box::new(Xorshift32RNG::new(seed));
    }
}
