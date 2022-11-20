use crate::utils::math::{get_n_bit_random_prime, get_d};
use serde::{Deserialize, Serialize};
use num_bigint::BigUint;

#[derive(Debug)]
pub struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicKey {
    pub public_exponent: BigUint,
    pub modulus: BigUint,
}

#[derive(Debug)]
pub struct PrivateKey {
    private_exponent: BigUint,
    modulus: BigUint,
}

impl KeyPair {
    // - generates rsa public-private keypair 
    // - input fermat number e 
    // - returns KeyPair
    //<<<<<TODO>>>Take e by reference>>>>>
    pub fn generate_key_pair(e:BigUint) -> KeyPair {
        let one = BigUint::from(1u32);
        loop {
            let p:BigUint = get_n_bit_random_prime(32);
            let q: BigUint = get_n_bit_random_prime(32);
            let n: BigUint = &p*&q;
            let phi: BigUint = (&p-&one)*(&q-&one);
            //<<<<<TODO>>>Make get_d take args by reference>>>>>
            let _e = e.clone();
            let d: BigUint = get_d(phi, _e);
            if d < BigUint::from(std::u32::MAX) {
                continue;
            }
            let pub_key = PublicKey {public_exponent: e, modulus: n.clone()};
            let priv_key = PrivateKey {private_exponent: d, modulus: n.clone()};
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

    pub fn from(e:BigUint, d:BigUint, n:BigUint) -> KeyPair {
        let public_key = PublicKey{public_exponent: e, modulus: n.clone()};
        let private_key = PrivateKey{private_exponent: d, modulus: n.clone()};
        return KeyPair{public_key, private_key};
    }
}

impl PublicKey {
    
    pub fn public_exponent(&self) -> &BigUint {
        return &self.public_exponent;
    }

    pub fn public_exponent_clone(&self) -> BigUint {
        self.public_exponent.clone()
    }

    pub fn modulus(&self) -> &BigUint {
        return &self.modulus;
    }

    pub fn modulus_clone(&self) -> BigUint {
        self.modulus.clone()
    }

}

impl PrivateKey {
    pub fn private_exponent(&self) -> &BigUint {
        &self.private_exponent
    }
    pub fn modulus(&self) -> &BigUint {
        &self.modulus
    }
}


