extern crate rsa_rs;

use num_bigint::BigUint;

use rsa_rs::keys::keypair::KeyPair;
use rsa_rs::encryption::encrypt::encrypt_string;
use rsa_rs::encryption::decrypt::decrypt_string;

#[test]
fn test_encryption() {
    let s = String::from("hello");
    let e = BigUint::from(65537u32);
    let key_pair = KeyPair::generate_key_pair(e, 32);
    let public_key = key_pair.public_key();
    let private_key = key_pair.private_key();
    dbg!(private_key);
    dbg!(public_key);
    let enc_vec = encrypt_string(&s, public_key);
    let dec_string = decrypt_string(&enc_vec, private_key);
    assert_eq!(s, dec_string);
    dbg!(dec_string);

}
