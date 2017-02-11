# prime_sieve
Tools for generating filters and collections with primes. 

Rust implementation of the Sieve of Atkin

## `prime_sieve::filter`
```Rust
fn prime_filter(iter_size: usize) -> Vec<bool>
```
>Generates a vector of bools of size iter_size, with `true` at each prime index, and `false` otherwise.

## `prime_sieve::collection`
```Rust
fn primes(max_prime: usize) -> Vec<usize>
```
>Generates a vector collection of primes strictly less than max_prime.
