//! Manager module for cryptography modules

use std::{str::Bytes, collections::HashMap};
use super::manager_rng;
pub mod xtea;

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
    rng: Box<dyn manager_rng::RNG>,
    key_lenght: u32,
    XTEAConfig: xtea::XTEAConfig,
}

// Impl for cryptography manager structure
impl CryptographyManager {
    pub fn new(rng: Box<dyn manager_rng::RNG>) -> Self {
        let xtea_config = xtea::XTEAConfig;
        CryptographyManager{
            encryption: Encryption::NONE,
            rng: rng,
            key_lenght: 0,
            XTEAConfig: xtea_config
        }
    }

    pub fn EC_encrypt(self) {
        
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