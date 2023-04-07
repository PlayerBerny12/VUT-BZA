//! Cryptographic library

#![allow(unused)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

use manager_crypt::{CryptographyManager, xtea, Encryption};
use manager_rng::RandomNumberGeneratorManager;

mod manager_crypt;
mod manager_rng;

/// High-level encryption method
pub fn encrypt(profile: &CryptographyManager, mut message: String, key: &xtea::Key) -> Vec<u32> {
    if profile.encryption == Encryption::XTEA {
        return profile.XTEA_encrypt(&mut message, key);
    }

    panic!("Not supported algorithm");
}

/// High-level decryption method
pub fn decrypt(profile: &CryptographyManager, mut message: Vec<u32>, key: &xtea::Key) -> String {
    if profile.encryption == Encryption::XTEA {
        return profile.XTEA_decrypt(&mut message, key);
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
        let seed: u32 = 5392047;
        let mut rng_mng: manager_rng::RandomNumberGeneratorManager = manager_rng::RandomNumberGeneratorManager::new_PseudoRNG(&seed, 0, 1);
        let mut rng_call_binding = &mut ||rng_mng.generator.next();
        let mut crypt_mng = manager_crypt::CryptographyManager::new(rng_call_binding);
        let key = crypt_mng.XTEA_generate_key();
        let message: String = String::from("Ahoj světe!....");
        
        let result = encrypt(&crypt_mng, message.clone(), &key);
        let result2 = decrypt(&crypt_mng, result, &key);
                
        assert_eq!(message, result2);
    }
}
