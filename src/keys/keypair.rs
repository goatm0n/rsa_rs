use std::sync::mpsc;
use std::thread;

use serde::{Deserialize, Serialize};
use num_bigint::BigUint;

use crate::utils::math::{get_n_bit_random_prime, get_d};
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

impl KeyPair {
    // - generates rsa public-private keypair 
    // - input fermat number e 
    // - returns KeyPair
    //<<<<<TODO>>>Take e by reference>>>>>
    pub fn generate_key_pair(e:BigUint, bits: u32) -> KeyPair {
        let one = BigUint::from(1u32);
        let first_primes = cached_primes();
        loop {
            let p:BigUint = get_n_bit_random_prime(bits.clone(), &first_primes);
            let q: BigUint = get_n_bit_random_prime(bits.clone(), &first_primes);
            let n: BigUint = &p*&q;
            let phi: BigUint = (&p-&one)*(&q-&one);
            //<<<<<TODO>>>Make get_d take args by reference>>>>>
            let d: BigUint = get_d(phi, e.clone());
            // <<<ISSUE>>> infinite loop when bits is small
            if d < BigUint::from(std::u32::MAX) {
                continue;
            }
            let pub_key = PublicKey {public_exponent: e, modulus: n.clone()};
            let priv_key = PrivateKey {private_exponent: d, modulus: n.clone()};
            let key_pair = KeyPair {public_key: pub_key, private_key: priv_key};
            break key_pair;
        }
    }

    pub fn concurrently_generate_key_pair(e:BigUint, bits:u32) -> KeyPair {
        let (tx, rx) = mpsc::channel();

        for _ in 0..2 {
            let e1 = e.clone();
            let bits1 = bits.clone();
            let tx1 = tx.clone();
            thread::spawn(move || {
                let key_pair = KeyPair::generate_key_pair(e1, bits1);
                let res = tx1.send(key_pair);
                match res {
                    Ok(()) => {},
                    Err(_) => {},
                }
            });
        }

        rx.recv().unwrap()

    }

    pub fn concurrently_generate_key_pair_threaded(e:BigUint, bits:u32) -> KeyPair {
        let (tx, rx) = mpsc::channel();

        for _ in 0..2 {
            let e1 = e.clone();
            let bits1 = bits.clone();
            let tx1 = tx.clone();
            thread::spawn(move || {
                let key_pair = KeyPair::generate_key_pair_threaded(e1, bits1);
                let res = tx1.send(key_pair);
                match res {
                    Ok(()) => {},
                    Err(_) => {},
                }
            });
        }

        rx.recv().unwrap()

    }

    pub fn generate_key_pair_threaded(e:BigUint, bits: u32) -> KeyPair {
        let one = BigUint::from(1u32);
        let first_primes = cached_primes();
        let (tx, rx) = mpsc::channel();

        loop {
            let first_primes1 = first_primes.clone();
            let tx1 = tx.clone();
            let tx2 = tx.clone();
            // spawn a thread to calculate p
            thread::spawn(move || {
                let p:BigUint = get_n_bit_random_prime(bits.clone(), &first_primes1);
                tx1.send(p).unwrap();
            });
            let q: BigUint = get_n_bit_random_prime(bits.clone(), &first_primes);
            let p = rx.recv().unwrap();
            let _q = q.clone();
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

    pub fn thread_gen_key(e: BigUint, bits: u32) ->  KeyPair {
        let first_primes = cached_primes();
        let one = BigUint::from(1u32);
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        
        thread::spawn(move || {
            loop {
                let p:BigUint = get_n_bit_random_prime(bits.clone(), &first_primes);
                let q: BigUint = get_n_bit_random_prime(bits.clone(), &first_primes);
                let n: BigUint = &p*&q;
                let phi: BigUint = (&p-&one)*(&q-&one);
                //<<<<<TODO>>>Make get_d take args by reference>>>>>
                let d: BigUint = get_d(phi, e.clone());
                // <<<ISSUE>>> infinite loop when bits is small
                if d < BigUint::from(std::u32::MAX) {
                    continue;
                }
                let key_pair = KeyPair::from(e, d, n);
                tx1.send(key_pair).unwrap();
                break;
            };
        });     

        let res = rx.recv().unwrap(); 
        res
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

#[cfg(test)]
mod tests {
    use super::KeyPair;
    use num_bigint::BigUint;

    #[test]
    fn test_key_gen() {
        let key = KeyPair::generate_key_pair(BigUint::from(65537u32), 128);
        dbg!(key);
    }

    #[test]
    fn test_threaded_key_gen() {
        let key = KeyPair::thread_gen_key(BigUint::from(65537u32), 128);
        dbg!(key);
    }

    #[test]
    fn test_generate_key_pair_threaded() {
        let key = KeyPair::generate_key_pair_threaded(BigUint::from(65537u32), 128);
        dbg!(key);
    }

    #[test]
    fn test_concurrently_generate_key_pair() {
        let key = KeyPair::concurrently_generate_key_pair(BigUint::from(655373u32), 128);
        dbg!(key);
    }

    #[test]
    fn test_concurrently_generate_key_pair_threaded() {
        let key = KeyPair::concurrently_generate_key_pair_threaded(BigUint::from(655373u32), 128);
        dbg!(key);
    }

}

