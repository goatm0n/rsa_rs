use crate::utils::math::{get_n_bit_random_prime, get_d};

#[derive(Debug)]
pub struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

#[derive(Debug)]
pub struct PublicKey {
    public_exponent: u128,
    modulus: u128,
}

#[derive(Debug)]
pub struct PrivateKey {
    private_exponent: u128,
    modulus: u128,
}

impl KeyPair {
    // - generates rsa public-private keypair 
    // - input fermat number e 
    // - returns KeyPair
    //
    pub fn generate_key_pair(e:u128) -> KeyPair {
        loop {
            let p:u128 = get_n_bit_random_prime(32);
            let q: u128 = get_n_bit_random_prime(32);
            let n: u128 = p*q;
            let phi: u128 = (p-1)*(q-1);
            let d = get_d(phi, e);
            if d < std::u32::MAX as u128 {
                continue;
            }
            let pub_key = PublicKey {public_exponent: e, modulus: n};
            let priv_key = PrivateKey {private_exponent: d, modulus: n};
            let key_pair = KeyPair {public_key: pub_key, private_key: priv_key};
            break key_pair;
        }
    }
    
    pub fn public_key(&self) -> &PublicKey {
        return &self.public_key;
    }

    pub fn private_key(&self) -> &PrivateKey {
        return &&self.private_key;
    }

    pub fn from(e:u128, d:u128, n:u128) -> KeyPair {
        let public_key = PublicKey{public_exponent: e, modulus: n};
        let private_key = PrivateKey{private_exponent: d, modulus: n};
        return KeyPair{public_key, private_key};
    }
}

impl PublicKey {
    
    pub fn public_exponent(&self) -> &u128 {
        return &self.public_exponent;
    }

    pub fn public_exponent_clone(&self) -> u128 {
        self.public_exponent.clone()
    }

    pub fn modulus(&self) -> &u128 {
        return &self.modulus;
    }

    pub fn modulus_clone(&self) -> u128 {
        self.modulus.clone()
    }

}

impl PrivateKey {
    pub fn private_exponent(&self) -> &u128 {
        &self.private_exponent
    }
    pub fn modulus(&self) -> &u128 {
        &self.modulus
    }
}


