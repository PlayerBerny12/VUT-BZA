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
}

// PLACEHOLDER for get_next_rand funciton
fn next() -> u32 {
    1
}