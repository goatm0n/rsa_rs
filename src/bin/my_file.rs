extern crate rsa;

use crate::rsa::utils::math::test;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    println!("hello from main");
    test();
}

