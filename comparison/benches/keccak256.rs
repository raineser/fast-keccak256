#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn tiny_keccak_keccak_256_input_32_bytes(b: &mut Bencher) {
    use tiny_keccak::{Hasher, Keccak};
    let data = vec![254u8; 32];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = [0; 32];
        let mut keccak = Keccak::v256();
        keccak.update(&data);
        keccak.finalize(&mut res);
    });
}

#[bench]
fn tiny_keccak_keccak_256_input_4096_bytes(b: &mut Bencher) {
    use tiny_keccak::{Hasher, Keccak};
    let data = vec![254u8; 4096];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = [0; 32];
        let mut keccak = Keccak::v256();
        keccak.update(&data);
        keccak.finalize(&mut res);
    });
}

#[bench]
fn fast_keccak_keccak_256_input_32_bytes(b: &mut Bencher) {
    use fast_keccak256::{keccak256};
    let data = vec![254u8; 32];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = keccak256(&data);
    });
}

#[bench]
fn fast_keccak_keccak_256_input_4096_bytes(b: &mut Bencher) {
    use fast_keccak256::{keccak256};
    let data = vec![254u8; 4096];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = keccak256(&data);
    });
}