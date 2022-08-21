use rand::Rng;

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
    if p > 1 {
        for i in 2..p {
            if p % i == 0 {
                return false
            } 
        }
        return true
    } else {
        return false
    }
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
    let a:u32 = (p-1)/2;
    let b:u32 = (q-1)/2;
    if is_prime(a) && is_prime(b) {
        return true;
    } 
    return false;
}
// - choose random e from safe primes p and q
// - panic if p and q are not safe primes
//
pub fn choose_random_e(p:u32, q:u32) -> u32 {
    if !(safe_primes(p, q)) {
        panic!("unsafe primes"); // should probably improve this
    }
    let phi_n:u32 = (p-1)*(q-1);
    let mut rng = rand::thread_rng();
    let mut i:u32 = rng.gen_range(2..phi_n/2);
    if i % 2 == 0 {
        i += 1;
    }
    let e:u32 = loop {
        dbg!(&i);
        if !(is_prime(i)) {
            i+=2; 
            continue;
        } 
        if is_coprime(i, phi_n) {
            break i;
        }
        i += 2;
        
    };
    return e;
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

pub fn mod_pow(mut base:u32,mut exp:u32, modulus:u32) -> u32 {
    if modulus == 1 {
        return 0;
    }
    let mut result:u32 = 1;
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

pub fn test() {
    // enforce safe primes
    let p = 23;
    let q = 59;
    let e = choose_random_e(p, q);
    dbg!(e);
}

