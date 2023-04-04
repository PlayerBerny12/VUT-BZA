//! Cryptographic library

#![allow(unused)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

use manager_crypt::CryptographyManager;
use manager_rng::RandomNumberGeneratorManager;

mod manager_crypt;
mod manager_rng;

// static CRYPTOGRAPHY_PROFILE: CryptographyManager;
// static RANDOM_PROFILE: RandomNumberGeneratorManager = null();

/// Setting global profile for cryptography
pub fn set_cryptography_profile(profile: CryptographyManager) {}

/// Setting global profile for random number generators
pub fn set_random_profile(profile: RandomNumberGeneratorManager) {}

/// High-level encryption method
pub fn encrypt() {}

/// High-level decryption method
pub fn decrypt() {}

/// High-level method for generating random numbers
pub fn random() {}
