use filter::prime_filter;

pub fn primes(max_prime: usize) -> Vec<usize> {
    let pf = prime_filter(max_prime);
    pf.iter().enumerate().filter_map(|(i, is_prime)|
        match is_prime {
            &true => Some(i),
            _ => None
        }).collect()
}
