VERSION 0.2.0 BREAKING CHANGE
-> Introduces num-bigint types 
-> Functions formerly taking u128 args now take BigUint
-> exponent and modulus values inside inside keypairs now represented as BigUints
-> generate much larger primes

VERSION 0.3.0 BREAKING CHANGE
-> KeyPair::generate_key_pair now takes 'bits' argument
-> KeyPair::generate_key_pair performance increased by threading