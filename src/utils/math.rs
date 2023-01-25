use num_bigint::{BigUint, BigInt, ToBigUint, RandBigInt};
use std::thread;
use std::sync::mpsc;

pub fn extended_euclidian(a:BigUint, b:BigUint) -> (BigUint, BigInt, BigInt) {
    if a == BigUint::from(0u32) {
        (b, BigInt::from(0u32), BigInt::from(1u32)) 
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

        (g, y-(_b/_a)*&x, x) 
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
    true
}

/// returns true if p, q coprime;
/// 
/// 
pub fn is_coprime(p:u128, q:u128) -> bool {
    gcd(p, q) == 1
}

fn gcd(p:u128, q:u128) -> u128 {
    if p == 0 {
        return q;
    }
    gcd(q % p, p)
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
    _d
}

pub fn mod_pow(mut base:BigUint, mut exp:BigUint, modulus:BigUint) -> BigUint {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if modulus == one {
        return two;
    }
    let mut result = one.clone();
    base %= &modulus;
    while exp > zero {
        if &exp % &two == one {
            result = &result * &base % &modulus;
        }
        exp >>= 1;
        base = &base * &base % &modulus;
    }
    result 
}

// - generate a n-bit random number;
//      - input n: u32 number of bits
//      - returns random integer: u128
//
pub fn n_bit_random(n:&u32) -> BigUint {
    let two = BigUint::from(2u32);
    let mut rng = rand::thread_rng();
    // returns a random number between 2^(n-1)+1 and 2^(n)-1
    let low = two.pow(n-1)+1.to_biguint().unwrap();
    let high = two.pow(n.to_owned())-1.to_biguint().unwrap();
    rng.gen_biguint_range(&low, &high)
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
        if prime[p] {
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
    for (i, val) in prime.iter().enumerate() {
        if *val {
            primes.push(i.try_into().unwrap());
        }
    }
    primes
} 

// - generate a prime candidate divisible by first primes 
//      - input n: u32 number of bits
//      - return prime number: usize
//
fn get_low_level_prime(n:&u32, first_primes: &[BigUint]) -> BigUint {
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

fn trial_composite(
    round_tester: BigUint, 
    max_divisions_by_2: u32, 
    even_component: BigUint, 
    miller_rabin_candidate: BigUint
) -> bool {
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if mod_pow(
        round_tester.clone(), 
        even_component.clone(), 
        miller_rabin_candidate.clone()
    ) == one {
        return false;
    }
    for i in 0..max_divisions_by_2 {
        let exp = two.pow(i) * &even_component;
        if mod_pow(
            round_tester.clone(), 
            exp, 
            miller_rabin_candidate.clone()
        ) == &miller_rabin_candidate - &one {
            return false;
        }
    }
    true
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
    // primality tests do not delete
    let test1 = two.pow(max_divisions_by_2) * &even_component;
    let test2 = miller_rabin_candidate - &one;
    assert_eq!(test1, test2);
    // set number of primes here
    let number_of_rabin_trials = 20;
    for _i in 0..number_of_rabin_trials {
        let round_tester = rng.gen_biguint_range(
            &two, 
            miller_rabin_candidate
        );
        if trial_composite(
            round_tester, 
            max_divisions_by_2, 
            even_component.clone(), 
            miller_rabin_candidate.clone()
        ) {
            return false;
        }
    }
    true
}

pub fn get_n_bit_random_prime(n:&u32, first_primes: &[BigUint]) -> BigUint {
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
pub fn thread_get_n_bit_random_prime(n:&u32, n_threads:u32, first_primes: &Vec<BigUint>) -> BigUint {
    let (tx, rx) = mpsc::channel();
    let n = n.to_owned();
    let first_primes = first_primes.to_owned();
    for _ in 0..n_threads {
        let tx = tx.clone();
        let n = n.to_owned();
        let first_primes = first_primes.to_owned();
        thread::spawn(move || {
            let res = get_n_bit_random_prime(&n, &first_primes);
            tx.send(res).unwrap();
        });
    }
    rx.recv().unwrap()
}


#[cfg(test)]
mod tests {
    use std::time::Instant;

    use num_bigint::{BigUint, RandBigInt};

    use crate::utils::math::trial_composite;

    use super::{
        sieve_of_eratosthenes,
        is_prime,
        get_n_bit_random_prime,
        thread_get_n_bit_random_prime, get_low_level_prime, miller_rabin, mod_pow,
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
        let n = 32u32;
        let p = get_n_bit_random_prime(&n, &first_primes);
        dbg!(&p);
        // this can get very computationally heavy for larger numbers
        //assert_eq!(true, is_prime(p));
    }

    #[test]
    fn test_thread_get_n_bit_random_prime() {
        let first_primes = sieve_of_eratosthenes(1000);
        let n = 32u32;
        let p = thread_get_n_bit_random_prime(&n, 4, &first_primes);
        dbg!(&p);
        // this can get very computationally heavy for larger numbers
        //assert_eq!(true, is_prime(p));
    }
    
    /* #[test]
    fn test_concurrently_get_n_bit_random_prime() {
        let p = concurrently_get_n_bit_random_prime(32);
        assert_eq!(true, is_prime(p)); 
    } */

    #[test]
    fn debug_get_n_bit_random_prime() {
        let n: &u32 = &1024;
        let binding = sieve_of_eratosthenes(100000);
        let first_primes = binding.as_slice();

        let mut loop_num = 0;
        let t_p_0 = Instant::now();
        let p = loop {
            loop_num += 1;
            println!("loop_num: {}", loop_num);
            let t_loop_0 = Instant::now();

            let t_get_p_0 = Instant::now();
            let miller_rabin_candidate = get_low_level_prime(n, first_primes);
            let t_get_p_1 = Instant::now();
            let t_get_p = t_get_p_1 - t_get_p_0;
            println!("t_get_p: {:#?}", t_get_p);

            let t_miller_0 = Instant::now();
            let miller_rabin = miller_rabin(&miller_rabin_candidate);
            let t_miller_1 = Instant::now();
            let t_miller = t_miller_1 - t_miller_0;
            println!("t_miller: {:#?}", t_miller);

            let t_loop_1= Instant::now();
            let t_loop = t_loop_1 - t_loop_0;
            println!("t_loop: {:#?}", t_loop);
        
            if !miller_rabin {
                continue;
            } else {
                break miller_rabin_candidate;
            }
        };
        let t_p_1 = Instant::now();
        let t_p = t_p_1 - t_p_0;
        println!("p: {} \nGenerated in: {:#?}", p, t_p);
    }

    #[test]
    fn debug_miller_rabin() {
        let miller_rabin_candidate = &BigUint::from(669544539825604973674715208601u128);

        let t0 = Instant::now();

        let one = BigUint::from(1u32);
        let two = BigUint::from(2u32);
        let zero = BigUint::from(0u32);

        let mut max_divisions_by_2 = 0;
        let mut even_component = miller_rabin_candidate - &one;
        let mut rng = rand::thread_rng(); 

        let t_while_loop_0 = Instant::now();
        while &even_component % &two == zero {
            even_component >>= 1;
            max_divisions_by_2 += 1;
        }
        let t_while_loop_1 = Instant::now();
        let t_while_loop = t_while_loop_1 - t_while_loop_0;
        println!("t_while_loop: {:#?}", t_while_loop);

        let t_primality_tests_0 = Instant::now();
        // primality tests do not delete
        let test1 = two.pow(max_divisions_by_2) * &even_component;
        let test2 = miller_rabin_candidate - &one;
        assert_eq!(test1, test2);
        let t_primality_tests_1 = Instant::now();
        let t_primality_tests = t_primality_tests_1 - t_primality_tests_0;
        println!("t_primality_tests: {:#?}", t_primality_tests);

        // set number of primes here
        let number_of_rabin_trials = 20;

        let mut res = true;

        let mut for_loop_iter = 0;
        let t_for_loop_0 = Instant::now();
        for _i in 0..number_of_rabin_trials {
            for_loop_iter += 1;
            println!("for_loop_iter: {:#?}", &for_loop_iter);
            let t_for_loop_iter_0 = Instant::now();

            let t_gen_biguint_0 = Instant::now();
            let round_tester = rng.gen_biguint_range(
                &two, 
                miller_rabin_candidate
            );
            let t_gen_biguint_1 = Instant::now();
            let t_gen_biguint = t_gen_biguint_1 - t_gen_biguint_0;
            println!("t_gen_biguint: {:#?}", t_gen_biguint);

            println!(
                "trail_composite args:\n\t
                round_tester: {}\n\t
                max_divisions_by_2: {}\n\t
                even_component: {}\n\t
                miller_rabin_candidate: {}", 
                &round_tester, 
                &max_divisions_by_2, 
                &even_component, 
                &miller_rabin_candidate
            );

            let t_trial_composite_0 = Instant::now();
            let trial_composite = trial_composite(
                round_tester, 
                max_divisions_by_2, 
                even_component.clone(), 
                miller_rabin_candidate.clone()
            );
            let t_trial_composite_1 = Instant::now();
            let t_trial_composite = t_trial_composite_1 - t_trial_composite_0;
            println!("t_trial_composite: {:#?}", t_trial_composite);

            if trial_composite {
                res = false;
                let t_for_loop_iter_1 = Instant::now();
                let t_for_loop_iter = t_for_loop_iter_1 - t_for_loop_iter_0;
                println!("t_for_loop_iter: {:#?}", t_for_loop_iter);
                let t1 = Instant::now();
                let t = t0 - t1;
                println!("Result: {}\nTime: {:#?}", res, t);
                return;
            }
            let t_for_loop_iter_1 = Instant::now();
            let t_for_loop_iter = t_for_loop_iter_1 - t_for_loop_iter_0;
            println!("t_for_loop_iter: {:#?}", &t_for_loop_iter);
        }
        let t_for_loop_1 = Instant::now();
        let t_for_loop = t_for_loop_1 - t_for_loop_0;
        println!("t_for_loop: {:#?}", t_for_loop);

        let t1 = Instant::now();
        let t = t1 - t0;
        println!("Result: {}\nTime: {:#?}", res, t);
        return;
    }

    #[test]
    fn debug_trial_composite() {
        let round_tester:BigUint = BigUint::from(439384002558036958644382685821u128); 
        let max_divisions_by_2: u32 = 3; 
        let even_component: BigUint = BigUint::from(83693067478200621709339401075u128);
        let miller_rabin_candidate: BigUint = BigUint::from(669544539825604973674715208601u128);

        let one = BigUint::from(1u32);
        let two = BigUint::from(2u32);

        let t0 = Instant::now();

        let mut res = true;

        let t_mod_pow_even_0 = Instant::now();
        let mod_pow_even = mod_pow(
            round_tester.clone(), 
            even_component.clone(), 
            miller_rabin_candidate.clone()
        );
        let t_mod_pow_even_1 = Instant::now();
        let t_mod_pow_even = t_mod_pow_even_1 - t_mod_pow_even_0;
        println!("t_mod_pow_even: {:#?}", t_mod_pow_even);

        if mod_pow_even == one {
            res = false;
            let t1 = Instant::now();
            let t = t1 - t0;
            println!("Result: {}\nTime: {:#?}", res, t);
            return;
        }
        for i in 0..max_divisions_by_2 {
            let exp = two.pow(i) * &even_component;

            let t_mod_pow_exp_0 = Instant::now();
            let mod_pow_exp = mod_pow(
                round_tester.clone(), 
                exp, 
                miller_rabin_candidate.clone()
            );
            let t_mod_pow_exp_1 = Instant::now();
            let t_mod_pow_exp = t_mod_pow_exp_1 - t_mod_pow_exp_0;
            println!("t_mod_pow_exp: {:#?}", t_mod_pow_exp);

            if mod_pow_exp == &miller_rabin_candidate - &one {
                res = false;
                let t1 = Instant::now();
                let t = t1 - t0;    
                println!("Result: {}\nTime: {:#?}", res, t);
                return;
            }
        }
        println!("Result: {}", res);
    }

    #[test]
    fn debug_mod_pow() {
        let mut base: BigUint = BigUint::from(439384002558036958644382685821u128); 
        let mut exp: BigUint = BigUint::from(83693067478200621709339401075u128); 
        let modulus: BigUint = BigUint::from(669544539825604973674715208601u128);

        let zero = BigUint::from(0u32);
        let one = BigUint::from(1u32);
        let two = BigUint::from(2u32);

        if modulus == one {
            println!("Result: {}", two);
            return;
        }
        let mut result = one.clone();

        let mut while_loop_iter = 0;
        let t0 = Instant::now();
        base %= &modulus;
        while exp > zero {
            while_loop_iter += 1;
            let t_while_loop_iter_0 = Instant::now();
            if &exp % &two == one {
                result = &result * &base % &modulus;
            }
            exp >>= 1;
            base = &base * &base % &modulus;
            let t_while_loop_iter_1 = Instant::now();
            let t_while_loop_iter = t_while_loop_iter_1 - t_while_loop_iter_0;
            println!("while_loop_iter: {}\nt_while_loop_iter: {:#?}", while_loop_iter, t_while_loop_iter);
        }
        let t1 = Instant::now();
        let t = t1 - t0;
        println!("Result: {}\nTime: {:#?}", result, t);
    } 

    #[test]
    fn assert_eq_mod_pow() {
        let base = BigUint::from(439384002558036958644382685821u128); 
        let exp: BigUint = BigUint::from(83693067478200621709339401075u128); 
        let modulus: BigUint = BigUint::from(669544539825604973674715208601u128);
        let t0 = Instant::now();
        let res1 = mod_pow(base.clone(), exp.clone(), modulus.clone());
        let t1 = Instant::now();
        let t = t1 - t0;
        println!("t1: {:#?}", t);

        let t0 = Instant::now();
        let res2 = BigUint::modpow(&base, &exp, &modulus);
        let t1 = Instant::now();
        let t = t1 - t0;
        println!("t2: {:#?}", t);

        assert_eq!(res1, res2);
    }


}






