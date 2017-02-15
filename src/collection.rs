use filter::prime_filter_section;

pub fn primes(max_prime: usize) -> Vec<usize> {
    primes_section(0, max_prime)
}
pub fn primes_section(min_prime: usize, max_prime: usize) -> Vec<usize> {
    let pf = prime_filter_section(min_prime, max_prime);
    pf.iter().enumerate().filter_map(|(i, is_prime)|
        match is_prime {
            &true => Some(i + min_prime),
            _ => None
        }).collect()
}
