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
    return gcd(p, q) == 1 
}

fn gcd(p:u32, q:u32) -> u32 {
    let mut p_temp = p;
    let mut q_temp = q;
    while q_temp != 0 {
        p_temp= q_temp;
        q_temp = p_temp%q_temp;
    }
    return p_temp
}

pub fn choose_random_e(n:u32, phi:u32) -> u32 {
    let mut rng = rand::thread_rng();
    let mut rand_int:u32 = rng.gen_range(0..phi);
    let e:u32 = loop {
        if is_prime(rand_int) && !(is_coprime(rand_int, n) && is_coprime(rand_int, phi)){
            break rand_int;
        }
        rand_int = rng.gen_range(0..phi);
    }; 
    return e
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




