//! Manager module for cryptography modules

use std::{str::Bytes, collections::HashMap};
use super::manager_rng;
pub mod xtea;

// A place to register new encryption names
#[derive(Debug)]
enum Encryption {
    NONE,
    #[cfg(feature = "xtea")]
    XTEA,
}

// Cryptography manager structure
pub struct CryptographyManager {
    encryption: Encryption,
    rng: Box<dyn manager_rng::RNG>,
    key_lenght: u32,
    #[cfg(feature = "xtea")]
    XTEAConfig: xtea::XTEAConfig,
}

// Impl for cryptography manager structure
impl CryptographyManager {
    pub fn new(rng: Box<dyn manager_rng::RNG>) -> Self {
        #[cfg(feature = "xtea")]
        let xtea_config = xtea::XTEAConfig;
        CryptographyManager{
            encryption: Encryption::NONE,
            rng: rng,
            key_lenght: 0,
            #[cfg(feature = "xtea")]
            XTEAConfig: xtea_config
        }
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_encrypt(self, message: &mut Bytes, key: xtea::Key) -> bool {
        false
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_decrypt(self, message: &mut Bytes, key: xtea::Key) -> bool {
        false
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_generate_key(self) -> xtea::Key {
        xtea::gen_key(&|| self.rng.next())
    }
}