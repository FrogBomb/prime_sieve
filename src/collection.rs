use filter::prime_filter_section_sequentially;
use concurrent_help::to_concurrent_on_section;
use num_cpus;
use primal_sieve;

pub fn primes(max_num: usize) -> Vec<usize> {
    primes_concurrently(max_num, num_cpus::get())
}
pub fn primes_section(min_num: usize, max_num: usize) -> Vec<usize> {
    primes_section_concurrently(min_num, max_num, num_cpus::get())
}
pub fn primes_sequentially(max_num: usize) -> Vec<usize> {
    primal_sieve::Sieve::new(max_num).primes_from(0).take_while(|&x| x<max_num).collect()
    // primes_section_sequentially(0, max_num)
}
pub fn primes_section_sequentially(min_num: usize, max_num: usize) -> Vec<usize> {
    if min_num == 0{
        return primes_sequentially(max_num);
    }
    let pf = prime_filter_section_sequentially(min_num, max_num);
    pf.iter().enumerate().filter_map(|(i, is_prime)|
        match is_prime {
            &true => Some(i + min_num),
            _ => None
        }).collect()
}
pub fn primes_concurrently(max_num:usize, threads:usize) -> Vec<usize>{
    primes_section_concurrently(0, max_num, threads)
}
pub fn primes_section_concurrently(min_num:usize, max_num:usize, threads:usize) -> Vec<usize>{
    to_concurrent_on_section(primes_section_sequentially, min_num, max_num, threads)
}
