# unknown_order ![Crates.io]((https://img.shields.io/crates/v/unknown_order.svg)](https://crates.io/crates/unknown_order)) ![License-Image](https://img.shields.io/badge/License-Apache%202.0-green.svg)

Crate for handling groups of unknown order.

I've seen this commonly across multiple projects where they need a multiprecision library
and use one of three libraries: [Gnu MP BigNum Library](https://gmplib.org/), [OpenSSL's BigNum Library](https://www.openssl.org/docs/man1.0.2/man3/bn.html)
and [Rust's BigInt Library](https://crates.io/crates/num-bigint), depending on the needs and requirements (licensing, performance, platform target, constant time).

This library wraps them all into a common API, so they can be used interchangeably.

Groups of unknown order require using a modulus that is the composite of two big prime numbers. This
library is designed to facilitate these use cases such as RSA, [Paillier](https://link.springer.com/content/pdf/10.1007%2F3-540-48910-X_16.pdf), [Hyperelliptic Curves](https://eprint.iacr.org/2020/196),
[Accumulators](https://eprint.iacr.org/2018/1188), [CL signatures](http://cs.brown.edu/people/alysyans/papers/camlys02b.pdf).

The modulus is not known at compile time which excludes using certain traits like `ff::PrimeField`, so
unfortunately, the caller needs to remember to use methods prefixed with `mod` to achieve the desired results.

This library can only have one implementation active at a time. Mixing between implementations isn't necessarily a
problem as much as injecting lots of dependencies and mixing licenses which is not a good idea. 
This also forces the user to understand what tradeoffs they are making when they select a specific implementation.
For example, some implementations may not be constant time versus others which is important when used for cryptographic purposes.

## Examples

```rust
use unknown_order::BigNumber;

fn main() {
    // Create a safe group of unknown order
    let p = BigNumber::safe_prime(1024);
    let q = BigNumber::safe_prime(1024);
    let n = p.clone() * q.clone();
    
    // Simulate RSA algorithm, DO NOT USE totally insecure
    
    // Public key
    let e = BigNumber::from(65537);
    
    // throw away when done
    let totient = (p.clone() - 1) * (q.clone() - 1);
    
    // Secret key
    let d = e.invert(&totient).unwrap();
    
    
}
```