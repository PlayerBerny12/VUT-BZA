//! Cryptographic library

#![allow(unused)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

mod manager_crypt;

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
    fn it_works_A() {

        use std::{num::Wrapping};
        let mut key = [Wrapping(0u32), Wrapping(0u32), Wrapping(0u32), Wrapping(0u32)];
        let mut v0 = Wrapping(0u32);
		let mut v1 = Wrapping(0u32);
		let mut sum = Wrapping(0u32);

		for _ in 0..64 as u32 {
			v0 += (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (sum + key[(sum.0 & 3) as usize]);
			sum += Wrapping(0x9E3779B9);
			v1 += (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (sum + key[((sum.0 >> 11) & 3) as usize]);
		}

		println!("{} {}",  v0.0, v1.0);
        assert!(1==1);
    }
}

// PLACEHOLDER for get_next_rand funciton
fn next() -> u32 {
    1
}