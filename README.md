
# concurrent_prime_sieve [![Build Status](https://travis-ci.org/FrogBomb/prime_sieve.svg?branch=master)](https://travis-ci.org/FrogBomb/prime_sieve)
Tools for generating filters and collections with primes concurrently.

Rust implementation of the Sieve of Atkin.

## `concurrent_prime_sieve::filter`
```Rust
fn prime_filter(iter_size: usize) -> Vec<bool>
```
>Generates a vector of bools of size iter_size, with `true` at each prime index, and `false` otherwise.

```Rust
fn prime_filter_section(min:usize, max: usize) -> Vec<bool>
```
>Similar to `fn prime_filter`, but just for numbers between min and max, returned in a vector of length max-min.

## `concurrent_prime_sieve::collection`
```Rust
fn primes(max_prime: usize) -> Vec<usize>
```
>Generates a vector collection of primes strictly less than max_prime.

```Rust
fn primes_section(min_prime: usize, max_prime: usize) -> Vec<usize>
```
>Generates a vector collection of primes between min_prime and max_prime.

```Rust
fn primes_concurrently(max_prime:usize, threads:usize) -> Vec<usize>
```
>Similar to `fn primes`, but can be performed across a specified number of threads.
>_(This can easilly cut the time by an order of magnitude)_

```Rust
fn primes_section_concurrently(min_prime:usize, max_prime:usize, threads:usize) -> Vec<usize>
```
>Similar to `fn primes_section`, but can be performed across a specified number of threads
>_(This can easilly cut the time by an order of magnitude)_
