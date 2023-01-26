use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use num_bigint::BigUint;

extern crate rsa_rs;
use rsa_rs::{
    utils::{
        math::{
            sieve_of_eratosthenes, 
            mod_pow, 
            trial_composite, 
            trial_composite_biguint_modpow,
            miller_rabin,
            miller_rabin_biguint_modpow,
            get_n_bit_random_prime,
            get_n_bit_random_prime_biguint_modpow,
        },
        io::cached_primes
    },
    keys::keypair::KeyPair
};

#[allow(dead_code)]
fn bench_key_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_gen_group");
    let bits = black_box(1024u32);
    let e = black_box(BigUint::from(65537u32)); 
    group.significance_level(0.1).sample_size(10);
    group.bench_function(
        "key_gen",
        |b| b.iter(|| KeyPair::generate_key_pair(e.clone(), bits))
    );
    group.finish();
}

#[allow(dead_code)]
fn bench_key_gen_biguint_modpow(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_gen_group");
    let bits = black_box(1024u32);
    let e = black_box(BigUint::from(65537u32)); 
    group.significance_level(0.1).sample_size(10);
    group.bench_function(
        "key_gen_biguint_modpow",
        |b| b.iter(|| KeyPair::generate_key_pair_biguint_modpow(e.clone(), bits))
    );
    group.finish();
}

#[allow(dead_code)]
fn bench_cached_primes(c: &mut Criterion) {
    c.bench_function(
        "cached_primes", 
        |b| b.iter(|| cached_primes("primelist.txt"))
    );
}

#[allow(dead_code)]
fn bench_sieve_of_eratosthenes(c: &mut Criterion) {
    c.bench_function(
        "sieve_of_eratosthenes", 
        |b| b.iter(|| sieve_of_eratosthenes(100000))
    );
}

#[allow(dead_code)]
fn bench_mod_pow(c: &mut Criterion) {
    c.bench_function(
        "mod_pow", 
        |b| b.iter( || mod_pow(
            BigUint::from(43938400255803695864438268582112345u128), 
            BigUint::from(8369306747820062170933940107512345u128), 
            BigUint::from(66954453982560497367471520860112345u128)
        ))
    );
}

#[allow(dead_code)]
fn bench_biguint_mod_pow(c: &mut Criterion) {
    c.bench_function(
        "biguint_mod_pow", 
        |b| b.iter(|| BigUint::modpow(
            &BigUint::from(43938400255803695864438268582112345u128), 
            &BigUint::from(8369306747820062170933940107512345u128), 
            &BigUint::from(66954453982560497367471520860112345u128)
        ))
    );
}

#[allow(dead_code)]
fn bench_trial_composite(c: &mut Criterion) {
    c.bench_function(
        "trial_composite", 
        |b| b.iter(|| trial_composite(
            BigUint::from(439384002558036958644382685821u128),
            3,
            BigUint::from(83693067478200621709339401075u128),
            BigUint::from(669544539825604973674715208601u128)
        ))
    );
}

#[allow(dead_code)]
fn bench_trial_composite_biguint_modpow(c: &mut Criterion) {
    c.bench_function(
        "trial_composite_biguint_modpow", 
        |b| b.iter(|| trial_composite_biguint_modpow(
            &BigUint::from(439384002558036958644382685821u128),
            3,
            &BigUint::from(83693067478200621709339401075u128),
            &BigUint::from(669544539825604973674715208601u128)
        ))
    );
}

#[allow(dead_code)]
fn bench_miller_rabin(c: &mut Criterion) {
    let two = BigUint::from(2u32);
    let miller_rabin_candidate = &BigUint::from(669544539825604973674715208601u128) * two;
    c.bench_function(
        "miller_rabin", 
        |b| b.iter(|| miller_rabin(
            &miller_rabin_candidate 
        ))
    );
}

#[allow(dead_code)]
fn bench_miller_rabin_biguint_modpow(c: &mut Criterion) {
    let two = BigUint::from(2u32);
    let miller_rabin_candidate = &BigUint::from(669544539825604973674715208601u128) * two;
    c.bench_function(
        "miller_rabin_biguint_modpow", 
        |b| b.iter(|| miller_rabin_biguint_modpow(
            &miller_rabin_candidate
        ))
    );
}

#[allow(dead_code)]
fn bench_get_n_bit_random_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_prime_group");
    let binding = cached_primes("primelist.txt");
    let first_primes = binding.as_slice();
    group.significance_level(0.1).sample_size(10);
    group.bench_function(
        "get_n_bit_random_prime", 
        |b| b.iter(|| get_n_bit_random_prime(
            &1024,
            first_primes
        ))
    );
}

#[allow(dead_code)]
fn bench_get_n_bit_random_prime_biguint_modpow(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_prime_biguint_modpow_group");
    let binding = cached_primes("primelist.txt");
    let first_primes = binding.as_slice();
    group.significance_level(0.1).sample_size(10);
    group.bench_function(
        "get_n_bit_random_prime_biguint_modpow", 
        |b| b.iter(|| get_n_bit_random_prime_biguint_modpow(
            &1024,
            first_primes
        ))
    );
}

criterion_group!(benches, bench_key_gen, bench_key_gen_biguint_modpow);
criterion_main!(benches);