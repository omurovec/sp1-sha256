//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);
use sha::utils::Digest;
use sha2::{Digest as _, Sha256};

pub fn main() {
    let data = sp1_zkvm::io::read::<String>();
    let digest = Sha256::digest(&data.as_bytes());
    let digest = Digest::try_from(digest.as_slice()).unwrap();
    sp1_zkvm::io::write(&digest);
}
