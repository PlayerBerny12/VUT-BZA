//! Manager module for cryptography modules

use std::str::Bytes;

pub mod xtea;
use super::manager_rng;

// A place to register new encryption names
#[derive(Debug)]
enum Encryption {
    NONE,
    RSA,
    XTEA,
}

// Cryptography manager structure
pub struct CryptographyManager {
    encryption: Encryption,
    rng: Box<dyn manager_rng::RNG>
}

// Impl for cryptography manager structure
impl CryptographyManager {
    pub fn new(rng: Box<dyn manager_rng::RNG>) -> Self {
        CryptographyManager{encryption: Encryption::NONE, rng: rng}
    }

    pub fn XTEA_encrypt(self, message: &mut Bytes, key: xtea::Key) -> bool {
        false
    }

    pub fn XTEA_decrypt(self, message: &mut Bytes, key: xtea::Key) -> bool {
        false
    }

    pub fn XTEA_generate_key(self) -> xtea::Key {
        xtea::gen_key(&|| self.rng.next())
    }
}