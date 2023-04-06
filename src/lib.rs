//! Cryptographic library

#![allow(unused)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

use manager_crypt::{CryptographyManager, xtea, Encryption};
use manager_rng::RandomNumberGeneratorManager;

mod manager_crypt;
mod manager_rng;

/// High-level encryption method
pub fn encrypt(profile: CryptographyManager, mut message: String, key: xtea::Key) -> Vec<u32> {
    if profile.encryption == Encryption::XTEA {
        profile.XTEA_encrypt(&mut message, key);
    }

    panic!("Not supported algorithm");
}

/// High-level decryption method
pub fn decrypt(profile: CryptographyManager, mut message: Vec<u32>, key: xtea::Key) -> String {
    if profile.encryption == Encryption::XTEA {
        profile.XTEA_decrypt(&mut message, key);
    }

    panic!("Not supported algorithm");
}

/// High-level method for generating random numbers
pub fn random(mut profile: RandomNumberGeneratorManager) -> u32 {
    profile.generator.next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        
    }
}
