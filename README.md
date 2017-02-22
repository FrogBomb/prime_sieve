
# concurrent_prime_sieve (v 0.2.2) [![Build Status](https://travis-ci.org/FrogBomb/prime_sieve.svg?branch=master)](https://travis-ci.org/FrogBomb/prime_sieve)
Tools for generating filters and collections with primes concurrently.

Rust implementation of the Sieve of Atkin.

This implementation runs in O(sqrt(max_num)) for finding fixed-sized sections of primes.

Therefore, this implementation will run faster than any implementation of the
Sieve of Eratosthenes with the same number of cores as the number of cores increases,
making it ideal for parallel processing.

## `concurrent_prime_sieve::filter`
```Rust
fn prime_filter(max_num: usize) -> Vec<bool>
```
>Generates a vector of bools of size max_num, with `true` at each prime index, and `false` otherwise.
>
>The number of threads is based on the number of virtual cores detected.

```Rust
fn prime_filter_concurrently(max_num: usize, threads: usize) -> Vec<bool>
```
>Similar to `fn prime_filter`, but allows for a custom number of threads.

```Rust
fn prime_filter_sequentially(max_num: usize) -> Vec<bool>
```
>Similar to `fn prime_filter`, but does not spawn any new threads.

```Rust
fn prime_filter_section(min_num:usize, max_num: usize) -> Vec<bool>
```
>Similar to `fn prime_filter`, but just for numbers between min and max, returned in a vector of length max-min.

```Rust
fn prime_filter_section_concurrently(min_num: usize, max_num: usize, threads: usize) -> Vec<bool>
```
>Similar to `fn prime_filter_section`, but allows for a custom number of threads.

```Rust
fn prime_filter_section_sequentially(min_num: usize, max_num: usize) -> Vec<bool>
```
>Similar to `fn prime_filter_section`, but does not spawn any new threads.

## `concurrent_prime_sieve::collection`
```Rust
fn primes(max_num: usize) -> Vec<usize>
```
>Generates a vector collection of primes strictly less than max_num.
>
>The number of threads is based on the number of virtual cores detected.

```Rust
fn primes_concurrently(max_num:usize, threads:usize) -> Vec<usize>
```
>Similar to `fn primes`, but allows for a custom number of threads.

```Rust
fn primes_sequentially(max_num: usize) -> Vec<usize>
```
>Similar to `fn primes`, but does not spawn any new threads.

```Rust
fn primes_section(min_num: usize, max_num: usize) -> Vec<usize>
```
>Generates a vector collection of primes between min_num and max_num.
>
>The number of threads is based on the number of virtual cores detected.

```Rust
fn primes_section_concurrently(min_num:usize, max_num:usize, threads:usize) -> Vec<usize>
```
>Similar to `fn primes_section`, but allows for a custom number of threads.

```Rust
fn primes_section_sequentially(min_num:usize, max_num:usize, threads:usize) -> Vec<usize>
```
>Similar to `fn primes_section`, but does not spawn any new threads.
