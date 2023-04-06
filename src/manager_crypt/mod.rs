//! Manager module for cryptography modules

use super::manager_rng;
use std::{collections::HashMap, str::Bytes};
pub mod xtea;

// A place to register new encryption names
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Encryption {
    NONE,
    #[cfg(feature = "xtea")]
    XTEA,
}

// Cryptography manager structure
pub struct CryptographyManager<'a> {
    pub encryption: Encryption,
    // rng: Box<dyn manager_rng::RNG>,
    rng: &'a dyn Fn() -> u32,
    key_lenght: u32,
    #[cfg(feature = "xtea")]
    XTEAConfig: xtea::XTEAConfig,
}

// Impl for cryptography manager structure
impl<'a> CryptographyManager<'a> {
    pub fn new(rng: &'a dyn Fn() -> u32) -> Self {
        #[cfg(feature = "xtea")]
        let xtea_config = xtea::XTEAConfig {
            number_of_rounds: 128,
        };
        CryptographyManager {
            encryption: Encryption::NONE,
            rng: rng,
            key_lenght: 0,
            #[cfg(feature = "xtea")]
            XTEAConfig: xtea_config,
        }
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_encrypt(self, message: &mut String, key: xtea::Key) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        let mut bytesBuffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut dataBuffer: [u32; 2] = [0,0];
        let mut index: usize = 0;

        for byte in message.as_bytes() {
            bytesBuffer[index] = *byte;
            index += 1;

            if index == 8 {
                index = 0;
                dataBuffer[0] = u32::from_ne_bytes(bytesBuffer[0..4].try_into().unwrap());
                dataBuffer[1] = u32::from_ne_bytes(bytesBuffer[4..8].try_into().unwrap());
                xtea::encrypt(self.XTEAConfig.number_of_rounds, &mut dataBuffer, &key);

                result.push(dataBuffer[0]);
                result.push(dataBuffer[1]);
            }
        }

        result
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_decrypt(self, message: &mut Vec<u32>, key: xtea::Key) -> String {
        let mut result: String = String::from("");  
        let mut bytesBuffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut dataBuffer: [u32; 2] = [0,0];
        let mut index: usize = 0;

        for chunk in message {
            dataBuffer[index] = *chunk;
            index += 1;

            if index == 2 {
                index = 0;
                xtea::decrypt(self.XTEAConfig.number_of_rounds, &mut dataBuffer, &key);

                for i in 0..2 {
                    bytesBuffer[4 * i..][..4].copy_from_slice(&dataBuffer[i].to_ne_bytes());
                }
            }
        }

        result
    }

    #[cfg(feature = "xtea")]
    pub fn XTEA_generate_key(self) -> xtea::Key {
        xtea::gen_key(self.rng)
    }
}
