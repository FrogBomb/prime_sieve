
# concurrent_prime_sieve [![Crate](https://img.shields.io/crates/v/concurrent_prime_sieve.svg)](https://crates.io/crates/concurrent_prime_sieve) [![Build Status](https://travis-ci.org/FrogBomb/prime_sieve.svg?branch=master)](https://travis-ci.org/FrogBomb/prime_sieve)
Tools for generating filters and collections with primes concurrently.

Rust implementation of the Sieve of Atkin.

This implementation runs in O( sqrt(max_num) + section_size ) with O( section_size ) memory.
(where section_size = max_num - min_num)

This implementation built to work well with parallel processing, distributed computing tasks, or if a reasonably large section of primes is desired within a range that is roughly quadratic to the size of the section. (e.g., If you need primes in a block of size 10^9 starting at roughly 10^18.) By integrating Huon Wilson's primal package (see below), this package also is able to more quickly calculate all primes below a desired threshold through concurrency.

With this algorithm, there is no need to calculate smaller primes or to have any communication between threads, as each section of primes is calculated completely independently. Therefore, it is efficient for and very easily implemented in a distributed system.

*This package takes advantage of the (very good) primal package by Huon Wilson (located here: https://crates.io/crates/primal) to compute the edge case with primes starting from 0. This is simply faster than using just the algorithm here, which is built for concurrency. The algorithm implemented there is approximately 12 times faster when calculating primes from 0 than this implementation. This package uses that to cut the time to find the same number of primes by 12/(cores+11).*

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
> _(Note: This is just the vector cast of primal_sieve::Sieve.)_

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
>_(Note: for min_num <= 210, this is just the vector cast of primal_sieve::Sieve.)

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
> _(Note: This is just a vector cast of the primal_sieve::SievePrimes.)_

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
> _(Note: For min_num <= 210, this is just a vector cast of the primal_sieve::SievePrimes.)_
