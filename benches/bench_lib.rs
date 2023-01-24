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
        math::sieve_of_eratosthenes,
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

criterion_group!(benches, bench_key_gen);
criterion_main!(benches);