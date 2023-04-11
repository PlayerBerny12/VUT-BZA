//! eXtended Tiny Encryption Algorithm module
// Reference for implementation: https://en.wikipedia.org/wiki/XTEA

use std::num::Wrapping;

pub struct XTEAConfig{
    pub number_of_rounds: u32
}

#[derive(Debug)]
pub struct Key(pub [u32; 4]);

// Encrypts data using num_rounds of rounds of XTEA algorithm using key
pub fn encrypt(num_rounds: u32, data: &mut [u32; 2], key: &Key) {
    let mut v0: Wrapping<u32> = Wrapping(data[0]);
    let mut v1: Wrapping<u32> = Wrapping(data[1]);
    let mut sum: Wrapping<u32> = Wrapping(0);
    let mut delta: Wrapping<u32> = Wrapping(0x9E3779B9);

    for _ in 0..num_rounds {
        v0 += ((((v1 << 4) ^ (v1 >> 5)) + v1) ^ (Wrapping(sum.0) + Wrapping(key.0[(sum.0 & 3) as usize])));
        sum += delta;
        v1 += ((((v0 << 4) ^ (v0 >> 5)) + v0)
            ^ (Wrapping(sum.0) + Wrapping(key.0[((sum.0 >> 11) & 3) as usize])));
    }

    data[0] = v0.0;
    data[1] = v1.0;
}

// Decrypts data using num_rounds of rounds of XTEA algorithm using key
pub fn decrypt(num_rounds: u32, data: &mut [u32; 2], key: &Key) {
    let mut v0: Wrapping<u32> = Wrapping(data[0]);
    let mut v1: Wrapping<u32> = Wrapping(data[1]);
    let mut delta: Wrapping<u32> = Wrapping(0x9E3779B9);
    let mut sum: Wrapping<u32> = Wrapping(delta.0) * Wrapping(num_rounds);

    for _ in 0..num_rounds {
        v1 -=
            (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (Wrapping(sum.0) + Wrapping(key.0[((sum.0 >> 11) & 3) as usize]));
        sum -= delta;
        v0 -= (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (Wrapping(sum.0) + Wrapping(key.0[(sum.0 & 3) as usize]));
    }

    data[0] = v0.0;
    data[1] = v1.0;
}

// Generates keypair for XTEA encryption/decryption
pub fn gen_key(next_rng: &mut dyn FnMut() -> u32) -> Key {
    let mut k = Key([0, 0, 0, 0]);
    for i in k.0.iter_mut() {
        *i = next_rng();
    }

    assert!(k.0 != [0, 0, 0, 0]);
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager_rng::xorshift::Xorshift32RNG;
    use crate::manager_rng::RNG;
    use std::sync::{Arc, Mutex};

    struct MockRNG {
        state: u32,
        rn: [u32; 4],
    }

    impl RNG for MockRNG {
        fn new(seed: &u32) -> MockRNG {
            MockRNG {
                state: *seed,
                rn: [165448890, 42994, 78941639, 96],
            }
        }

        fn next(&mut self) -> u32 {
            let value = self.rn[self.state as usize];
            self.state += 1;

            value
        }
    }

    #[test]
    fn test_keygen() {
        let seed = 0;
        let rngMut = Arc::new(Mutex::new(MockRNG::new(&seed)));

        let key = gen_key(&mut || rngMut.lock().unwrap().next());

        assert_eq!(key.0, [165448890, 42994, 78941639, 96])
    }

    #[test]
    fn test_encrypt_decrypt() {
        let data_out = [123456789, 987654321];
        let mut data = data_out;
        let key = Key([100, 200, 300, 400]);
        let num_rounds = 128;

        encrypt(num_rounds, &mut data, &key);

        assert_eq!(data, [2613847215, 3063251712]);

        decrypt(num_rounds, &mut data, &key);

        assert_eq!(data, data_out);
    }
}
