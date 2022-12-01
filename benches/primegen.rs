use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};

extern crate rsa_rs;
use rsa_rs::utils::math::{
    //thread_get_n_bit_random_prime,
    get_n_bit_random_prime
};
use rsa_rs::utils::io::cached_primes;


fn bench_prime_gen(c: &mut Criterion) {
    let first_primes = cached_primes();
    c.bench_function(
        "prime_gen",
        |b| b.iter(|| get_n_bit_random_prime(128, &first_primes))
    );
}

/* fn threaded_prime_gen(c: &mut Criterion) {
    let first_primes = cached_primes();
    c.bench_function(
        "threaded_prime_gen",
        |b| b.iter(|| thread_get_n_bit_random_prime(1024, 4, &first_primes))
    );
} */


criterion_group!(benches, bench_prime_gen);//, threaded_prime_gen);
criterion_main!(benches);