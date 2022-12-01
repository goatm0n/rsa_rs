use num_bigint::{BigUint, BigInt, ToBigUint, RandBigInt};
use std::thread;
use std::sync::mpsc;

pub fn extended_euclidian(a:BigUint, b:BigUint) -> (BigUint, BigInt, BigInt) {
    if a == BigUint::from(0u32) {
        return (b, BigInt::from(0u32), BigInt::from(1u32)) 
    }
    else {
        // unenjoyable >>> TODO >>> use references for extended_euclidian
        /* let _a = a.clone();
        let _b = b.clone();
        let a = &_b % &_a;
        let b = _a.clone(); */
        let _a = BigInt::from(a.clone());
        let _b = BigInt::from(b.clone());
        let (g, x, y) = extended_euclidian(b%&a, a);
        // some fuckery

        return (g, y-(_b/_a)*&x, x) 
    }
}

pub fn is_prime(p:BigUint) -> bool {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let four = BigUint::from(4u32);
    let five = BigUint::from(5u32);
    if p <= one || p == four {return false;} // covers non-primes 0,1 and 4
    if p <= five {return true;} // covers primes 2,3 and 5 (4 eliminated)
    // now p is >= 5
    for i in num_iter::range(five, p.clone()) {
        if &p % i == zero {
            return false;
        } 
    }
    return true;
}

/// returns true if p, q coprime;
/// 
/// 
pub fn is_coprime(p:u128, q:u128) -> bool {
    return gcd(p, q) == 1; 
}

fn gcd(p:u128, q:u128) -> u128 {
    if p == 0 {
        return q;
    }
    return gcd(q % p, p);
}

pub fn get_d(phi:BigUint, e:BigUint) -> BigUint {
    let zero = BigInt::from(0u32);
    let _phi = phi.clone();
    let (_g, k, mut d) = extended_euclidian(_phi, e);
    d = std::cmp::max(k, d);
    if d < zero {
        d += BigInt::from(phi);
    }
    assert!(d >= BigInt::from(0u32));
    let _d: BigUint = d.clone().try_into().unwrap();
    return _d
}

pub fn mod_pow(mut base:BigUint, mut exp:BigUint, modulus:BigUint) -> BigUint {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if modulus == one {
        return two;
    }
    let mut result = one.clone();
    base = base % &modulus;
    while exp > zero {
        if &exp % &two == one {
            result = &result * &base % &modulus;
        }
        exp = exp >> 1;
        base = &base * &base % &modulus;
    }
    result 
}

// - generate a n-bit random number;
//      - input n: u32 number of bits
//      - returns random integer: u128
//
pub fn n_bit_random(n:u32) -> BigUint {
    let two = BigUint::from(2u32);
    let mut rng = rand::thread_rng();
    // returns a random number between 2^(n-1)+1 and 2^(n)-1
    let low = two.pow(n-1)+1.to_biguint().unwrap();
    let high = two.pow(n)-1.to_biguint().unwrap();
    let rand_int = rng.gen_biguint_range(&low, &high);
    //let rand_int = rng.gen_range(low..high);
    return rand_int
}

/// -generate all primes <= n 
///      - n: uzise, max prime size
///      - return Vec<uzise> contiaing primes <= n
///
pub fn sieve_of_eratosthenes(n:usize) -> Vec<BigUint> {

    // create boolean array prime[0..n], init all as true
    let mut prime: Vec<bool> = vec![true; n+1];
    // set 0, 1 false
    prime[0] = false;
    prime[1] = false;
    // start iterating from p=2
    let mut p = 2;
    while p*p <= n {
        if prime[p] == true {
            // update all multiples of p >= p^2 
            // numbers which are multiple of p and < p^2 are already marked
            let mut i = p*p;
            while i <= n {
                prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    // create vector of primes
    let mut primes: Vec<BigUint> = Vec::new();
    // append if prime
    for i in 0..prime.len() {
        if prime[i] {
            primes.push(i.try_into().unwrap());
        }
    }
    return primes; 
} 

// - generate a prime candidate divisible by first primes 
//      - input n: u32 number of bits
//      - return prime number: usize
//
fn get_low_level_prime(n:u32, first_primes: &Vec<BigUint>) -> BigUint {
    let zero = BigUint::from(0u32);
    loop {
        let prime_candidate = n_bit_random(n);
        for divisor in first_primes {
            if &prime_candidate%divisor == zero && divisor.pow(2) <= prime_candidate {
                continue;
            } else {
                return prime_candidate;
            }        
        }
    }
}

fn trial_composite(round_tester:BigUint, max_divisions_by_2:u32, even_component:BigUint, miller_rabin_candidate: BigUint) -> bool {
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    // <<<<<<TODO >>> pass args to mod_pow by reference>>>>>>
    let _round_tester = round_tester.clone();
    let _max_divisions_by_2 = max_divisions_by_2.clone();
    let _even_component = even_component.clone();
    let _miller_rabin_candidate = miller_rabin_candidate.clone();
    let base = round_tester.clone();
    let exp = even_component.clone();
    let modulus = miller_rabin_candidate.clone();
    if mod_pow(base, exp, modulus) == one {
        return false;
    }
    for i in 0..max_divisions_by_2 {
        let base = _round_tester.clone();
        let exp = two.pow(i) * _even_component.clone();
        let modulus = _miller_rabin_candidate.clone();
        if mod_pow(base, exp, modulus) == &miller_rabin_candidate - &one {
            return false;
        }
    }
    return true;
}


fn miller_rabin(miller_rabin_candidate: &BigUint) -> bool {
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    let zero = BigUint::from(0u32);

    let mut max_divisions_by_2 = 0;
    let mut even_component = miller_rabin_candidate - &one;
    let mut rng = rand::thread_rng(); 
    while &even_component % &two == zero {
        even_component >>= 1;
        max_divisions_by_2 += 1;
    }
    // primality tests do no delete
    let test1 = two.pow(max_divisions_by_2) * &even_component;
    let test2 = miller_rabin_candidate - &one;
    assert_eq!(test1, test2);
    // set number of primes here
    let number_of_rabin_trials = 20;
    for _i in 0..number_of_rabin_trials {
        let round_tester = rng.gen_biguint_range(&two, miller_rabin_candidate);
        if trial_composite(round_tester, max_divisions_by_2.clone(), even_component.clone(), miller_rabin_candidate.clone()) {
            return false;
        }
    }
    return true;
}

pub fn get_n_bit_random_prime(n:u32, first_primes: &Vec<BigUint>) -> BigUint {
    loop {
        let miller_rabin_candidate = get_low_level_prime(n, first_primes);
        if !miller_rabin(&miller_rabin_candidate) {
            continue;
        } else {
            break miller_rabin_candidate;
        }
    }
}

/// Calls get_n_bit_random_prime on n_threads.
/// Returns value from first thread finished
pub fn thread_get_n_bit_random_prime(n:u32, n_threads:u32, first_primes: &Vec<BigUint>) -> BigUint {
    let (tx, rx) = mpsc::channel();
    for _ in 0..n_threads {
        let tx = tx.clone();
        let n = n.clone();
        let first_primes = first_primes.clone();
        thread::spawn(move || {
            let res = get_n_bit_random_prime(n, &first_primes);
            tx.send(res).unwrap();
        });
    }
    rx.recv().unwrap()
}


#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::{
        sieve_of_eratosthenes,
        is_prime,
        get_n_bit_random_prime,
        thread_get_n_bit_random_prime,
    };
    #[test]
    fn unit_test_is_prime_true() {
        assert_eq!(is_prime(BigUint::from(983u32)), true);
    }

    #[test]
    fn unit_test_is_prime_false() {
        assert_eq!(is_prime(BigUint::from(12u32)), false);
    }
    
    #[test]
    fn test_primality_sieve_of_eratosthenes () {
        let vec = sieve_of_eratosthenes(1000);
        for p in vec {
            assert_eq!(is_prime(p), true);
        }
    }

    #[test]
    fn test_get_n_bit_random_prime() {
        let first_primes = sieve_of_eratosthenes(1000);
        let p = get_n_bit_random_prime(32, &first_primes);
        dbg!(&p);
        // this can get very computationally heavy for larger numbers
        //assert_eq!(true, is_prime(p));
    }

    #[test]
    fn test_thread_get_n_bit_random_prime() {
        let first_primes = sieve_of_eratosthenes(1000);
        let p = thread_get_n_bit_random_prime(32, 4, &first_primes);
        dbg!(&p);
        // this can get very computationally heavy for larger numbers
        //assert_eq!(true, is_prime(p));
    }
    
    /* #[test]
    fn test_concurrently_get_n_bit_random_prime() {
        let p = concurrently_get_n_bit_random_prime(32);
        assert_eq!(true, is_prime(p)); 
    } */

}






