use rsa_rs::keys::keypair::KeyPair;
use num_bigint::BigUint;


#[test]
fn test_key_gen() {
    KeyPair::generate_key_pair(BigUint::from(65537u32),128);
}