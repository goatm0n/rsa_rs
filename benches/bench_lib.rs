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
        math::{sieve_of_eratosthenes, mod_pow},
        io::cached_primes
    },
    keys::keypair::KeyPair
};

#[allow(dead_code)]
fn bench_key_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_gen_group");
    let bits = black_box(256u32);
    let e = black_box(BigUint::from(65537u32)); 
    group.significance_level(0.1).sample_size(10);
    group.bench_function(
        "key_gen",
        |b| b.iter(|| KeyPair::generate_key_pair(e.clone(), bits))
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
            BigUint::from(439384002558036958644382685821123u128), 
            BigUint::from(83693067478200621709339401075123u128), 
            BigUint::from(669544539825604973674715208601123u128)
        ))
    );
}

#[allow(dead_code)]
fn bench_biguint_mod_pow(c: &mut Criterion) {
    c.bench_function(
        "biguint_mod_pow", 
        |b| b.iter(|| BigUint::modpow(
            &BigUint::from(439384002558036958644382685821123u128), 
            &BigUint::from(83693067478200621709339401075123u128), 
            &BigUint::from(669544539825604973674715208601123u128)
        ))
    );
}

criterion_group!(benches, bench_mod_pow, bench_biguint_mod_pow);
criterion_main!(benches);