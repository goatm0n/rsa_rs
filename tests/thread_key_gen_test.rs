use rsa_rs::utils::math::{
    thread_get_n_bit_random_prime, 
    sieve_of_eratosthenes,
    get_d,
};
use num_bigint::BigUint;
use rsa_rs::keys::keypair::KeyPair;

#[test]
pub fn test_threaded_key_gen() {
    let one = BigUint::from(1u32);
    let first_primes = sieve_of_eratosthenes(10000);
    let bits = 128u32;
    let e = BigUint::from(65537u32);
    let keypair = loop {
        let p:BigUint = thread_get_n_bit_random_prime(&bits, 4,&first_primes);
        let q: BigUint = thread_get_n_bit_random_prime(&bits, 4,&first_primes.clone());
        let n: BigUint = &p*&q;
        let phi: BigUint = (&p-&one)*(&q-&one);
        //<<<<<TODO>>>Make get_d take args by reference>>>>>
        let _e = e.clone();
        let d: BigUint = get_d(phi, _e);
        if d < BigUint::from(std::u32::MAX) {
            continue;
        }
        let key_pair = KeyPair::from(e, d, n);
        break key_pair;
    };
    dbg!(&keypair);
}