use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use num_bigint::BigUint;

extern crate rsa_rs;
use rsa_rs::keys::keypair::KeyPair;


fn bench_key_gen(c: &mut Criterion) {
    let bits = black_box(256u32);
    let e = black_box(BigUint::from(65537u32)); 
    c.bench_function(
        "key_gen",
        |b| b.iter(|| KeyPair::generate_key_pair(e.clone(), bits))
    );
}

fn bench_threaded_key_gen(c: &mut Criterion) {
    let bits = black_box(256u32);
    let e = black_box(BigUint::from(65537u32)); 
    c.bench_function(
        "key_gen_threaded",
        |b| b.iter(|| KeyPair::generate_key_pair_threaded(e.clone(), bits))
    );
}

fn bench_concurrent_key_gen(c: &mut Criterion) {
    let bits = black_box(256u32);
    let e = black_box(BigUint::from(65537u32)); 
    c.bench_function(
        "concurrent_key_gen",
        |b| b.iter(|| KeyPair::concurrently_generate_key_pair(e.clone(), bits))
    );
}

fn bench_concurrent_key_gen_threaded(c: &mut Criterion) {
    let bits = black_box(256u32);
    let e = black_box(BigUint::from(65537u32)); 
    c.bench_function(
        "concurrent_key_gen_threaded",
        |b| b.iter(|| KeyPair::concurrently_generate_key_pair_threaded(e.clone(), bits))
    );
}

criterion_group!(benches, bench_key_gen, bench_threaded_key_gen);
criterion_main!(benches);