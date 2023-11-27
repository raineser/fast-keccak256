# fast-keccak256

This is a implementation of the ['Keccak256'](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf) hash function with the goal of becoming the fastest rust based implementation. This is still very much a work in progress. 

## Benchmarks

This implementation is currently slightly faster than 'tiny-keccak' with a 32 byte input.

```
running 4 tests
test fast_keccak_keccak_256_input_32_bytes   ... bench:         246 ns/iter (+/- 1) = 130 MB/s
test fast_keccak_keccak_256_input_4096_bytes ... bench:       7,570 ns/iter (+/- 59) = 541 MB/s
test tiny_keccak_keccak_256_input_32_bytes   ... bench:         255 ns/iter (+/- 11) = 125 MB/s
test tiny_keccak_keccak_256_input_4096_bytes ... bench:       7,459 ns/iter (+/- 51) = 549 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 1.40s

```