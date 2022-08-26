use rand::Rng;
use std::cmp::max;

pub fn extended_euclidian(a:i32, b:i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1) 
    }
    else {
        let (g, x, y) = extended_euclidian(b%a, a);
        return (g, y-(b/a)*x, x) 
    }
}

pub fn is_prime(p:u32) -> bool {
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

pub fn is_coprime(p:u32, q:u32) -> bool {
    return gcd(p, q) == 1; 
}

fn gcd(p:u32, q:u32) -> u32 {
    if p == 0 {
        return q;
    }
    return gcd(q % p, p);
}

pub fn safe_primes(p:u32, q:u32) -> bool {
    if !(is_prime(p) && is_prime(q)) {
        return false;
    }
    let a:u32 = (p-1)/2;
    let b:u32 = (q-1)/2;
    if is_prime(a) && is_prime(b) {
        return true;
    } 
    return false;
}

pub fn choose_random_primes(e:u32) -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let mut p:u32;
    let mut q:u32;
    let mut phi_n:u32;
    loop {
        p = rng.gen_range(0..e);
        q = rng.gen_range(0..e);
        if !(safe_primes(p, q)) {
            continue;
        }
        phi_n = (p-1)*(q-1);
        if is_coprime(e, phi_n) && phi_n > e {
            return (p, q);
        }
    }
}

pub fn get_d(phi:u32, e:u32) -> u32 {
    let (_g, k, mut d) = extended_euclidian(phi.try_into().unwrap(), e.try_into().unwrap()) ;
    d = std::cmp::max(k, d);
    if d < 0 {
        let x:i32 = phi.try_into().unwrap();
        d += x;
    }
    return d.try_into().unwrap()
}

pub fn mod_pow(mut base:u64,mut exp:u64, modulus:u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result:u64 = 1;
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
//      - returns random integer: u32
//
pub fn n_bit_random(n:u32) -> u32 {
    let mut rng = rand::thread_rng();
    // returns a random number between 2^(n-1)+1 and 2^(n)-1
    let rand_int:u32 = rng.gen_range(2_u32.pow(n-1)+1..2_u32.pow(n)-1);
    return rand_int;
}

// -generate all primes <= n 
//      - n: uzise, max prime size
//      - return Vec<uzise> contiaing primes <= n
//
pub fn sieve_of_eratosthenes(n:usize) -> Vec<usize> {
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
    let mut primes: Vec<usize> = Vec::new();
    // append if prime
    for i in 0..prime.len() {
        if prime[i] {
            primes.push(i);
        }
    }
    // return vec<usize> primes 
    return primes;

    
}

// - generate a prime candidate divisible by first primes 
//      - input n: u32 number of bits
//      - return prime number: u32
//
//pub fn get_low_level_prime(n:u32) -> u32 {
    // repeat until a number satisfying the test isn't found
    //loop {
      //  prime_candidate = n_bit_random(n);
        //for divisor in first_primes_list {

  //      }
    //}
//}

pub fn test() {
    let vec = sieve_of_eratosthenes(100);
    dbg!(vec);
}
