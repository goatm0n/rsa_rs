use crate::keys::keypair::PublicKey;
use crate::utils::math::mod_pow;

//- encrypt string
//    - input: String, PublicKey 
//    - convert String to utf-8 byte array using String.as_bytes()
//    - encrypt utf-8 byte using PublicKey, modular exponentiation
//    - return encrypted utf-8 bytes as Vec<u32>
//
pub fn encrypt_string(s:&String, public_key:&PublicKey) -> Vec<u32> {
    let utf8_bytes:&[u8] = s.as_bytes();
    let public_exponent:&u32 = public_key.public_exponent();
    let modulus:&u32 = public_key.modulus();
    let mut base:u32;
    let mut encrypted_bytes: Vec<u32> = Vec::new();
    for byte in utf8_bytes.iter() {
        base = *byte as u32;
        let enc_byte:u32 = mod_pow(base.into(), (*public_exponent).into(), (*modulus).into()).try_into().unwrap();
        encrypted_bytes.push(enc_byte);
    }
    encrypted_bytes

} 
