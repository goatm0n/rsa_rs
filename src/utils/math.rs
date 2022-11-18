use rand::Rng;

pub fn extended_euclidian(a:i128, b:i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1) 
    }
    else {
        let (g, x, y) = extended_euclidian(b%a, a);
        return (g, y-(b/a)*x, x) 
    }
}

pub fn is_prime(p:u128) -> bool {
    if p <= 1 || p == 4 {return false;} // covers non-primes 0,1 and 4
    if p <= 3 {return true;} // covers primes 2 and 3
    // now p is >= 5
    for i in 5..p {
        if p % i == 0 {
            return false;
        } 
    }
    return true;
}

pub fn is_coprime(p:u128, q:u128) -> bool {
    return gcd(p, q) == 1; 
}

fn gcd(p:u128, q:u128) -> u128 {
    if p == 0 {
        return q;
    }
    return gcd(q % p, p);
}

pub fn get_d(phi:u128, e:u128) -> u128 {
    let (_g, k, mut d) = extended_euclidian(phi.try_into().unwrap(), e.try_into().unwrap()) ;
    d = std::cmp::max(k, d);
    if d < 0 {
        let x:i128 = phi.try_into().unwrap();
        d += x;
    }
    return d.try_into().unwrap()
}

pub fn mod_pow(mut base:u128, mut exp:u128, modulus:u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result:u128 = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus;
    }
    result 
}

// - generate a n-bit random number;
//      - input n: u32 number of bits
//      - returns random integer: u128
//
pub fn n_bit_random(n:u32) -> u128 {
    let mut rng = rand::thread_rng();
    // returns a random number between 2^(n-1)+1 and 2^(n)-1
    let rand_int:u128 = rng.gen_range(2_u128.pow(n-1)+1..2_u128.pow(n)-1);
    return rand_int;
}

// -generate all primes <= n 
//      - n: uzise, max prime size
//      - return Vec<uzise> contiaing primes <= n
//
pub fn sieve_of_eratosthenes(n:usize) -> Vec<u128> {
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
    let mut primes: Vec<u128> = Vec::new();
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
fn get_low_level_prime(n:u32) -> u128 {
    let first_primes = sieve_of_eratosthenes(10000);
    loop {
        let prime_candidate = n_bit_random(n);
        for divisor in &first_primes {
            if prime_candidate%divisor == 0 && divisor.pow(2) <= prime_candidate {
                continue;
            } else {
                return prime_candidate;
            }        
        }
    }
}

fn trial_composite(round_tester:u128, max_divisions_by_2:u32, even_component:u128, miller_rabin_candidate: u128) -> bool {
    if mod_pow(round_tester, even_component, miller_rabin_candidate) == 1 {
        return false;
    }
    for i in 0..max_divisions_by_2 {
        if mod_pow(round_tester, 2_u128.pow(i) * even_component, miller_rabin_candidate) == miller_rabin_candidate - 1 {
            return false;
        }
    }
    return true;
}

fn miller_rabin(miller_rabin_candidate: u128) -> bool {
    let mut max_divisions_by_2: u32 = 0;
    let mut even_component: u128 = miller_rabin_candidate - 1;
    let mut rng = rand::thread_rng(); 
    while even_component % 2 == 0 {
        even_component >>= 1;
        max_divisions_by_2 += 1;
    }
    assert_eq!(2_u128.pow(max_divisions_by_2) * even_component, miller_rabin_candidate - 1);
    // set number of primes here
    let number_of_rabin_trials = 20;
    for _i in 0..number_of_rabin_trials {
        let round_tester:u128 = rng.gen_range(2..miller_rabin_candidate);
        if trial_composite(round_tester, max_divisions_by_2, even_component, miller_rabin_candidate) {
            return false;
        }
    }
    return true;
}

pub fn get_n_bit_random_prime(n:u32) -> u128 {
    loop {
        let prime_candidate:u128 = get_low_level_prime(n) as u128;
        if !miller_rabin(prime_candidate) {
            continue;
        } else {
            break prime_candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        sieve_of_eratosthenes,
        is_prime,
        get_n_bit_random_prime
    };
    
    #[test]
    fn test_primality_sieve_of_eratosthenes () {
        let vec = sieve_of_eratosthenes(1000);
        for p in vec {
            assert_eq!(is_prime(p.try_into().unwrap()), true);
        }
    }

    #[test]
    fn test_get_n_bit_random_prime() {
        let p = get_n_bit_random_prime(16);
        assert_eq!(true, is_prime(p));
    }
    

}






