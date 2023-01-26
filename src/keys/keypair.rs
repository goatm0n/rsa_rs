use std::sync::mpsc;
use std::thread;

use serde::{Deserialize, Serialize};
use num_bigint::BigUint;

use crate::utils::math::{get_n_bit_random_prime, get_d, sieve_of_eratosthenes};
use crate::utils::io::cached_primes;


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

pub struct KeyPairError (KeyPairErrorKind);


pub enum KeyPairErrorKind {
    TIMEOUT,
}

impl Default for KeyPair {
    fn default() -> Self {
        Self::new()    
    }
}

impl KeyPair {
    pub fn new() -> Self {
        todo!()
    }

    pub fn try_generate_key_pair(bits: &u32) -> Self {
        let e = BigUint::from(65537u32);
        let first_primes = sieve_of_eratosthenes(100000);
        let first_primes = first_primes.as_slice();
        let p = get_n_bit_random_prime(bits, first_primes);
        let q = get_n_bit_random_prime(bits, first_primes);
        let n = &p * &q;
        let one = BigUint::from(1u32);
        let phi: BigUint = (&p-&one)*(&q-&one);
        let d = get_d(phi, e.clone());

        Self::from(e, d, n)
    }

    // - generates rsa public-private keypair 
    // - input fermat number e 
    // - returns KeyPair
    //<<<<<TODO>>>Take e by reference>>>>>
    pub fn generate_key_pair(e:BigUint, bits: u32) -> KeyPair {
        let one = BigUint::from(1u32);
        let first_primes = cached_primes("primelist.txt");
        let (tx, rx) = mpsc::channel();

        loop {
            let first_primes1 = first_primes.clone();
            let tx1 = tx.clone();
            let tx2 = tx.clone();
            // spawn a thread to calculate p
            thread::spawn(move || {
                let p:BigUint = get_n_bit_random_prime(&bits, &first_primes1);
                tx1.send(p).unwrap();
            });
            let q: BigUint = get_n_bit_random_prime(&bits, &first_primes);
            let _q = q.clone();
            let p = rx.recv().unwrap();
            let _p = p.clone();
            // spawn a thread to calculate n
            thread::spawn(move || {
                let n: BigUint = _p*_q;
                tx2.send(n).unwrap();
            });
            let phi: BigUint = (&p-&one)*(&q-&one);
            let d: BigUint = get_d(phi, e.clone());
            let n = rx.recv().unwrap();
            if d < BigUint::from(std::u32::MAX) {
                continue;
            }
            let key_pair = KeyPair::from(e, d, n);
            break key_pair
        }
    }
    
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    pub fn from(e:BigUint, d:BigUint, n:BigUint) -> KeyPair {
        let public_key = PublicKey{public_exponent: e, modulus: n.clone()};
        let private_key = PrivateKey{private_exponent: d, modulus: n};
        
        Self { public_key, private_key }
    }
}

impl PublicKey {
    
    pub fn public_exponent(&self) -> &BigUint {
        &self.public_exponent
    }

    pub fn public_exponent_clone(&self) -> BigUint {
        self.public_exponent.clone()
    }

    pub fn modulus(&self) -> &BigUint {
        &self.modulus
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

#[cfg(test)]
mod tests {
    use std::{time::Instant};

    use crate::utils::math::{sieve_of_eratosthenes, get_n_bit_random_prime, get_d};

    use super::KeyPair;
    use num_bigint::BigUint;

    #[test]
    fn test_key_gen() {
        let key = KeyPair::generate_key_pair(BigUint::from(65537u32), 128);
        dbg!(key);
    }

    #[test]
    fn debug_generate_key_pair() {
        let one = BigUint::from(1u32);
        let e = BigUint::from(65537u32);
        let bits = 1024u32;
        let first_primes = sieve_of_eratosthenes(100000);

        let t0 = Instant::now();

        let mut loop_num = 0;

        let key_pair = loop {
            loop_num += 1;
            println!("loop_num: {}", &loop_num);
            let t0 = Instant::now();

            let t_p_0 = Instant::now();
            let p:BigUint = get_n_bit_random_prime(&bits, &first_primes);
            let t_p_1 = Instant::now();
            let t_p = t_p_1 - t_p_0;
            println!("t_p: {:#?}", t_p);

            let t_q_0 = Instant::now();
            let q: BigUint = get_n_bit_random_prime(&bits, &first_primes);
            let t_q_1 = Instant::now();
            let t_q = t_q_1 - t_q_0;
            println!("t_q: {:#?}", t_q);

            let t_n_0 = Instant::now();
            let n: BigUint = &p*&q;
            let t_n_1 = Instant::now();
            let t_n = t_n_1 - t_n_0;
            println!("t_n: {:#?}", t_n);
    
            let t_phi_0 = Instant::now();
            let phi: BigUint = (&p-&one)*(&q-&one);
            let t_phi_1 = Instant::now();
            let t_phi = t_phi_1 - t_phi_0;
            println!("t_phi: {:#?}", t_phi);

            let t_get_d_0 = Instant::now();
            let d: BigUint = get_d(phi, e.clone());
            let t_get_d_1 = Instant::now();
            let t_get_d = t_get_d_1 - t_get_d_0;
            println!("get_d_time: {:#?}", t_get_d);

            let t1 = Instant::now();
            let t = t1 - t0;
            if d < BigUint::from(std::u32::MAX) {
                println!("loop_time: {:#?}", &t);
                continue;
            }
            println!("loop_time: {:#?}", t);
            break KeyPair::from(e, d, n);
        };

        let t1 = Instant::now();
        let t = t1 - t0;
        println!("KeyPair: {:?} \nGenerated in {:#?}", key_pair, t);

    }

}
