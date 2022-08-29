use crate::keys::keypair::PrivateKey;
use crate::utils::math::mod_pow;

//- decrypt bytes
//    - input: bytes Vec<u32>, PrivateKey
//    - decrypt bytes to utf-8 using PrivateKey and modular exponentiation
//    return utf-8 bytes as Vec<u8>
//    
pub fn decrypt_utf8(encrypted_utf8:&Vec<u128>, private_key:&PrivateKey) -> Vec<u8> {
    let exp:&u128= private_key.private_exponent();
    let modulus:&u128 = private_key.modulus();
    let mut decrypted_byte:u128;
    let mut utf8: u8;
    let mut utf8_vec: Vec<u8> = Vec::new();
    for byte in encrypted_utf8.iter() {
        let base = *byte as u128;
        decrypted_byte = mod_pow(base, (*exp).into(), (*modulus).into()).try_into().unwrap();
        utf8 = decrypted_byte as u8;
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
pub fn decrypt_string(encrypted_utf8:&Vec<u128>, private_key:&PrivateKey) -> String {
    let utf8_vec:Vec<u8> = decrypt_utf8(encrypted_utf8, &private_key);
    dbg!(&utf8_vec);
    let decrypted_string = String::from_utf8(utf8_vec);
    let result = match decrypted_string {
        Ok(my_string) => my_string,
        Err(error) => panic!("Problem converting to string: {:?}", error),
    };
    return result;
}





