use num_bigint::BigUint;
use crate::keys::keypair::PrivateKey;
use crate::utils::math::mod_pow;

//- decrypt bytes
//    - input: bytes Vec<u32>, PrivateKey
//    - decrypt bytes to utf-8 using PrivateKey and modular exponentiation
//    return utf-8 bytes as Vec<u8>
//    
pub fn decrypt_utf8(encrypted_utf8:&Vec<BigUint>, private_key:&PrivateKey) -> Vec<u8> {
    let exp:&BigUint = private_key.private_exponent();
    let modulus:&BigUint = private_key.modulus();
    let mut utf8_vec: Vec<u8> = Vec::new();
    for byte in encrypted_utf8.iter() {
        let base = byte.clone();
        let decrypted_byte = mod_pow(base, exp.clone(), modulus.clone());
        let utf8:u8 = decrypted_byte.try_into().expect("ERROR: Failure to cast decrypted bytes to u8");
        utf8_vec.push(utf8);
    }
    return utf8_vec;
}

// -decrypt to string
//    - input: encrypted bytes Vec<u32>, PrivateKey
//    - decrypt bytes using decrypt_utf8()
//    - construct string from decrypted bytes
//    - return string
//
pub fn decrypt_string(encrypted_utf8:&Vec<BigUint>, private_key:&PrivateKey) -> String {
    let utf8_vec:Vec<u8> = decrypt_utf8(encrypted_utf8, &private_key);
    let decrypted_string = String::from_utf8(utf8_vec);
    let result = match decrypted_string {
        Ok(my_string) => my_string,
        Err(error) => panic!("Problem converting to string: {:?}", error),
    };
    return result;
}





