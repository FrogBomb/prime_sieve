use filter::prime_filter_section;
use concurrent_help::to_concurrent_on_section;
use num_cpus;

pub fn primes(max_num: usize) -> Vec<usize> {
    primes_concurrently(max_num, num_cpus::get())
}
pub fn primes_section(min_num: usize, max_num: usize) -> Vec<usize> {
    primes_section_concurrently(min_num, max_num, num_cpus::get())
}
pub fn primes_sequentially(max_num: usize) -> Vec<usize> {
    primes_section(0, max_num)
}
pub fn primes_section_sequentially(min_num: usize, max_num: usize) -> Vec<usize> {
    let pf = prime_filter_section(min_num, max_num);
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
