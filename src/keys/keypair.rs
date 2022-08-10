use crate::utils::math::{choose_random_e, get_d};

#[derive(Debug)]
struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

#[derive(Debug)]
struct PublicKey {
    public_exponent: u32,
    modulus: u32,
}

#[derive(Debug)]
struct PrivateKey {
    private_exponent: u32,
    modulus: u32,
}

impl KeyPair {
    
    pub fn generate_key_pair(p: u32, q: u32) -> KeyPair {
        let n: u32 = p*q;
        let phi: u32 = (p-1)*(q-1);
        let e:u32 = choose_random_e(n, phi);
        println!("{e}");
        let d:u32 = get_d(phi, e);
        let pub_key = PublicKey {public_exponent: e, modulus: n};
        let priv_key = PrivateKey {private_exponent: d, modulus: n};
        let key_pair = KeyPair {public_key: pub_key, private_key: priv_key};
        return key_pair
    }

}


pub fn test() {
    println!("hello from keypair test()");
    let key_pair = KeyPair::generate_key_pair(13, 23);
    dbg!(key_pair);
}


