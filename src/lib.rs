//! Cryptographic library

#![allow(unused)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

mod manager_crypt;
mod manager_rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = manager_crypt::xtea::Key([0,0,0,0]);
        println!("{:#?}", key);
        let mut data = [0,0];
        manager_crypt::xtea::encrypt(64, &mut data, key);
        println!("{} {}", data[0], data[1]);
        // assert_eq!(key.0, [4, 4, 4, 4]);
        assert!(data[0]==4237446418);
        assert!(data[1]==1255206224);
    }


    #[test]
    fn xtea_keygen() {
        let seed: u32 = 1;
        let rng_mng: manager_rng::RandomNumberGeneratorManager = manager_rng::RandomNumberGeneratorManager::new_PseudoRNG(&seed, 0, 1);
        let crypt_mng = manager_crypt::CryptographyManager::new(rng_mng.generator);
        let key = crypt_mng.XTEA_generate_key();
        println!("{:#?}", key);
        
        assert!(key.0[0]==1);
    }
}