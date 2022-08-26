use crate::utils::math::{choose_random_primes, get_d};

#[derive(Debug)]
pub struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

#[derive(Debug)]
pub struct PublicKey {
    public_exponent: usize,
    modulus: usize,
}

#[derive(Debug)]
pub struct PrivateKey {
    private_exponent: usize,
    modulus: usize,
}

impl KeyPair {
    // - generates rsa public-private keypair 
    // - input fermat number e 
    // - returns KeyPair
    //
    pub fn generate_key_pair(e:usize) -> KeyPair {
        let (p, q) = choose_random_primes(e);
        dbg!(&p);
        dbg!(&q);
        let n: usize = p*q;
        let phi: usize = (p-1)*(q-1);
        let d:usize = get_d(phi, e);
        let pub_key = PublicKey {public_exponent: e, modulus: n};
        let priv_key = PrivateKey {private_exponent: d, modulus: n};
        let key_pair = KeyPair {public_key: pub_key, private_key: priv_key};
        return key_pair
    }
    
    pub fn public_key(&self) -> &PublicKey {
        return &self.public_key;
    }

    pub fn private_key(&self) -> &PrivateKey {
        return &&self.private_key;
    }
}

impl PublicKey {
    
    pub fn public_exponent(&self) -> &usize {
        return &self.public_exponent;
    }

    pub fn modulus(&self) -> &usize {
        return &self.modulus;
    }

}

impl PrivateKey {
    pub fn private_exponent(&self) -> &usize {
        &self.private_exponent
    }
    pub fn modulus(&self) -> &usize {
        &self.modulus
    }
}





