//! eXtended Tiny Encryption Algorithm module

use std::num::Wrapping;

pub struct XTEAConfig;

#[derive(Debug)]
pub struct Key(pub [u32; 4]);

// Encrypts data using num_rounds of rounds of XTEA algorithm using key
pub fn encrypt(num_rounds: u32, data: &mut [u32; 2], key: Key) {
    let mut v0: Wrapping<u32> = Wrapping(data[0]);
    let mut v1: Wrapping<u32> = Wrapping(data[1]);
    let mut sum: Wrapping<u32> = Wrapping(0);
    let mut delta: Wrapping<u32> = Wrapping(0x9E3779B9);

    for _ in 0..num_rounds {
        v0 += ((((v1 << 4) ^ (v1 >> 5)) + v1) ^ Wrapping(sum.0 + key.0[(sum.0 & 3) as usize]));
        sum += delta;
        v1 += ((((v0 << 4) ^ (v0 >> 5)) + v0)
            ^ Wrapping(sum.0 + key.0[((sum.0 >> 11) & 3) as usize]));
    }

    data[0] = v0.0;
    data[1] = v1.0;
}

// Decrypts data using num_rounds of rounds of XTEA algorithm using key
pub fn decrypt(num_rounds: u32, data: &mut [u32; 2], key: Key) {
    let mut v0: Wrapping<u32> = Wrapping(data[0]);
    let mut v1: Wrapping<u32> = Wrapping(data[1]);
    let mut delta: Wrapping<u32> = Wrapping(0x9E3779B9);
    let mut sum: Wrapping<u32> = Wrapping(delta.0 * num_rounds);

    for _ in 0..num_rounds {
        v1 -=
            (((v0 << 4) ^ (v0 >> 5)) + v0) ^ Wrapping(sum.0 + key.0[((sum.0 >> 11) & 3) as usize]);
        sum -= delta;
        v0 -= (((v1 << 4) ^ (v1 >> 5)) + v1) ^ Wrapping(sum.0 + key.0[(sum.0 & 3) as usize]);
    }

    data[0] = v0.0;
    data[1] = v1.0;
}

// Generates keypair for XTEA encryption/decryption
pub fn gen_key(next_rn: &dyn Fn() -> u32) -> Key {
    let mut k = Key([0, 0, 0, 0]);
    for i in k.0.iter_mut() {
        *i = next_rn();
    }

    assert!(k.0 != [0, 0, 0, 0]);
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let key = Key([0, 0, 0, 0]);
        println!("{:#?}", key);
        let mut data = [0, 0];
        encrypt(64, &mut data, key);
        println!("{} {}", data[0], data[1]);
        // assert_eq!(key.0, [4, 4, 4, 4]);
        assert!(data[0] == 4237446418);
        assert!(data[1] == 1255206224);
    }
}
