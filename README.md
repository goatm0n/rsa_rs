# [goatm0n]: rsa_rs

RSA implementation in Rust.

[![crates.io][crate-image]][crate-link]
[![Documentation][doc-image]][doc-link]
[![dependency status][deps-image]][deps-link]

VERSION 0.2.0 BREAKING CHANGE
-> Introduces num-bigint types 
-> Functions formerly taking u128 args now take BigUint
-> exponent and modulus values inside inside keypairs now represented as BigUints
-> generate much larger primes

VERSION 0.3.0 BREAKING CHANGE
-> KeyPair::generate_key_pair now takes 'bits' argument
-> KeyPair::generate_key_pair performance increased by threading

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/rsa_rs
[crate-link]: https://crates.io/crates/rsa_rs
[doc-image]: https://docs.rs/rsa_rs/badge.svg
[doc-link]: https://docs.rs/rsa_rs
[deps-image]: https://deps.rs/repo/github/goatm0n/rsa_rs/status.svg
[deps-link]: https://deps.rs/repo/github/goatm0n/rsa_rs
