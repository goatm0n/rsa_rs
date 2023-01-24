use num_bigint::BigUint;
use crate::keys::keypair::PublicKey;
use crate::utils::math::mod_pow;

//- encrypt string
//    - input: String, PublicKey 
//    - convert String to utf-8 byte array using String.as_bytes()
//    - encrypt utf-8 byte using PublicKey, modular exponentiation
//    - return encrypted utf-8 bytes as Vec<u32>
//
pub fn encrypt_string(s:&String, public_key:&PublicKey) -> Vec<BigUint> {
    let utf8_bytes:&[u8] = s.as_bytes();
    let public_exponent:BigUint= public_key.public_exponent_clone();
    let modulus:BigUint = public_key.modulus_clone();
    let mut encrypted_bytes: Vec<BigUint> = Vec::new();
    for byte in utf8_bytes.iter() {
        let base: BigUint = BigUint::from(*byte);
        let enc_byte: BigUint = mod_pow(base, public_exponent.clone(), modulus.clone());
        encrypted_bytes.push(enc_byte);
    }
    encrypted_bytes
}

