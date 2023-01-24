use std::path::PathBuf;
use num_bigint::BigUint;

use super::math::sieve_of_eratosthenes;


pub fn write_vec_biguint(path: &PathBuf, data: Vec<BigUint>) {
    let mut contents = String::new();
    for num in data {
        let num_string = num.to_string();
        contents += num_string.as_str();
        contents += "\t";
    }
    std::fs::write(path, contents).expect("error writing vector to file");
}

pub fn read_vec_biguint(path: &PathBuf) -> Vec<BigUint> {
    let file_content = std::fs::read_to_string(path).expect("could not read file");
    if file_content.is_empty() {panic!("empty-file: nothing to decrypt")}
    let mut num_string = String::new();
    let mut num_vec:Vec<BigUint> = Vec::new();
    for c in file_content.chars() {
        if c!= '\t' {
            num_string.push(c);
        } else {
            let num = num_string.parse::<BigUint>().unwrap();
            num_vec.push(num);
            num_string.clear();
        }
    }
    num_vec
}

pub fn cache_primes(n: usize, path: &PathBuf) {
    let primes = sieve_of_eratosthenes(n);
    write_vec_biguint(path, primes);
}

///  returns list of small primes from txt file
pub fn cached_primes(path: &str) -> Vec<BigUint> {
    read_vec_biguint(&PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::{cache_primes, read_vec_biguint, cached_primes};
    use crate::utils::math::sieve_of_eratosthenes;

    #[test]
    fn test_cache_primes() {
        let path = PathBuf::from("primelist.txt");
        cache_primes(100000, &path); 
    }

    #[test]
    fn test_read_primes() {
        let path = PathBuf::from("primelist.txt");
        cache_primes(100000, &path);
        let primes = read_vec_biguint(&path);
        dbg!(primes);
    }

    #[test]
    fn test_cached_primes() {
        assert_eq!(cached_primes("primelist.txt"), sieve_of_eratosthenes(100000));
    }
}