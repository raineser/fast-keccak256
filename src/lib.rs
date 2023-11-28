#![no_std]
use crunchy::unroll;
const RC:[u64; 24] = [0x0000000000000001,
                    0x0000000000008082,
                    0x800000000000808a,
                    0x8000000080008000,
                    0x000000000000808B,
                    0x0000000080000001,
                    0x8000000080008081,
                    0x8000000000008009,
                    0x000000000000008a,
                    0x0000000000000088,
                    0x0000000080008009,
                    0x000000008000000a,
                    0x000000008000808B,
                    0x800000000000008B,
                    0x8000000000008089,
                    0x8000000000008003,
                    0x8000000000008002,
                    0x8000000000000080,
                    0x000000000000800a,
                    0x800000008000000a,
                    0x8000000080008081,
                    0x8000000000008080,
                    0x0000000080000001,
                    0x8000000080008008];

const RHO:[u32; 24] = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44];

const PI:[usize; 24] = [2, 11, 7, 13, 18, 15, 1, 8, 16, 9, 24, 20, 3, 19, 23, 17, 12, 10, 4, 22, 14, 21, 6, 5];

//byte-aligned 10*1 padding with 01 delimter 
#[inline]
fn pad( bytes: &mut [u8; 200], m: usize) { 

    // q is the number of padding bytes needed 
    let q = 136 - (m % 136);
    bytes[m] = bytes[m] ^ 0x01;
    bytes[m + (q-2) + 1] =  bytes[m + (q-2) + 1] ^ 0x80;

}

#[inline]
fn to_little_endian(bytes: &[u8]) -> u64 {
    let mut x:u64 = 0;
    assert!(bytes.len() > 7);

    //unrolled 
    x += bytes[0] as u64;
    x += (bytes[1] as u64) << (8) as u64;
    x += (bytes[2] as u64) << (16) as u64;
    x += (bytes[3] as u64) << (24) as u64;
    x += (bytes[4] as u64) << (32) as u64;
    x += (bytes[5] as u64) << (40) as u64;
    x += (bytes[6] as u64) << (48) as u64;
    x += (bytes[7] as u64) << (56) as u64;

    x
}

#[inline]
fn to_big_endian(bits: u64, index: usize) -> u8 {

    let val: u8 = ((bits >> (8 * index)) % 256) as u8;
    val
}

// Converts s to state array a
#[inline]
fn to3d(s: &[u8; 200], a: &mut [u64; 25]) {

    for x in 0..5 {
        for y in 0..5 {
            // Endianness is flipped to little 
            a[(5*x)+y] = to_little_endian(&s[8*(x+5*y)..8*(x+5*y)+8]);
        }
    }
}
// Converts state array a to s
#[inline]
fn to1d(s: &mut [u8; 200], a: &mut [u64; 25]) {

    for x in 0..5 {
        for y in 0..5{
            unroll! {
                for byte in 0..8 {
                    // Endianness is flipped to big
                    s[8*(x+5*y)+byte] = to_big_endian(a[(5*x)+y], byte);
                }
            }
        }
    }
}

#[inline]
fn hash_round(s: &mut [u8; 200]) {
    // a state array 
    // A 1d array is faster than 2d
    let mut a: [u64; 25] = [0;25];

    let mut c = [0,0,0,0,0];

    let mut d: [u64; 5] = [0;5];

    let mut t: [u64; 5] = [0;5];

    to3d(s, &mut a);

    for round in 0..24 {
        
        //theta
        
       for x in 0..5 {
            c[x] = a[5*x] ^ a[(5*x)+1] ^ a[(5*x)+2]^ a[(5*x)+3]^ a[(5*x)+4];
       }
        
        unroll! {
            for x in 0..5 {
                d[x] = c[(x+4)% 5] ^ c[(x+1) % 5].rotate_left(1);
            }
        }

      
        for x in 0..5 {
            unroll! {
                for y in 0..5 {
                    a[(5*x)+y] ^= d[x];
                }
            }
        }

        //rho and pi
        let mut current = a[5];
        unroll! {
            for t in 0..24{
                (current, a[PI[t]]) = (a[PI[t]], current.rotate_left(RHO[t]));
            
            }
        }

        //chi
        for y in 0..5 {

            for x in 0..5 {
                d[x] = a[(5*x)+y];
            }
            unroll! {
                for x in 0..5 {
                    a[(5*x)+y] = d[x] ^ (( !d[(x+1)%5]) & d[(x+2)%5]);
                }
            }
        }
        
        //iota
        a[0] ^= RC[round];

    }
    to1d(s, &mut a);
}


pub fn keccak256(bytes: &[u8]) -> [u8; 32] {

    let mut s: [u8; 200] = [0; 200];

    let mut offset: usize = 0;

    if bytes.len() % 136 == 0 {
        for i in 0..(bytes.len()/136) as i64 -1{
            
            for j in 0..136 {
                s[j] = s[j] ^ bytes[(i as usize *136)+j]     
            }
            hash_round(&mut s);

            offset += 136;
        } 
        
    } else {
        for i in 0..bytes.len()/136 {
            for j in 0..136{
                s[j] =  s[j] ^ bytes[(i*136)+j];
            }
            hash_round(&mut s);
            
            offset += 136;
        }
    }

    for i in offset..bytes.len() {
        s[i-offset] = s[i-offset] ^ bytes[i];
    }

    // pad last block to 136 bytes
    pad(&mut s, bytes.len() - offset);

    hash_round(&mut s);

    let mut hash: [u8; 32] = [0; 32];

    hash.copy_from_slice(&s[..32]);

    hash

}
