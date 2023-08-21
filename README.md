# rsa_rs

[![crates.io][crate-image]][crate-link]
[![Documentation][doc-image]][doc-link]
[![dependency status][deps-image]][deps-link]

Rust library containing an implementation of the RSA cryptography algorithim.  \
Functionality: \
-> generate keypairs \
-> encrypt / decrypt strings \
-> generate random large primes \

## Example

```rust
use rsa_rs::keys::keypair::KeyPair;
use rsa_rs::encryption::encrypt::encrypt_string;
use rsa_rs::encryption::decrypt::decrypt_string;
use num_bigint::BigUint;

let key_pair: String = KeyPair::generate_key_pair(65537, 1024);
let public_key: &PublicKey = key_pair.public_key();
let private_key: &PrivateKey = key_pair.private_key();

let s = String::from("hello");
let encrypted_string: Vec<BigUint> = encrypt_string(&s, public_key);
let decrypted_string: String = decrypt_string(&encrypted_string, private_key);

assert_eq!(s, decrypted_string);

```

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/rsa_rs
[crate-link]: https://crates.io/crates/rsa_rs
[doc-image]: https://docs.rs/rsa_rs/badge.svg
[doc-link]: https://docs.rs/rsa_rs
[deps-image]: https://deps.rs/repo/github/goatm0n/rsa_rs/status.svg
[deps-link]: https://deps.rs/repo/github/goatm0n/rsa_rs
