use filter::prime_filter_section;

use std::thread;
use std::sync::mpsc;

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
pub fn primes_concurrently(max_prime:usize, threads:usize) -> Vec<usize>{
    primes_section_concurrently(0, max_prime, threads)
}
pub fn primes_section_concurrently(min_prime:usize, max_prime:usize, threads:usize) -> Vec<usize>{
    let mut res_vec: Vec<Vec<usize>> = vec![vec![]; threads];
    let seg_size = (max_prime - min_prime)/threads;
    let (tx, rx) = mpsc::channel();
    for i in 0..threads{
        let (tx, min, max) = (tx.clone(), min_prime + seg_size*i,
                                min_prime + seg_size*(i+1));
        thread::spawn( move || {
            tx.send((i, primes_section(min, max))).unwrap();
        });
    }
    if (min_prime + seg_size*threads) != max_prime {
        let (tx, min, max) = (tx.clone(), min_prime + seg_size*threads, max_prime);
        thread::spawn( move || {
            tx.send((threads, primes_section(min, max))).unwrap();
        });

        let (i, p_sec) = match rx.recv(){
            Ok(mes) => mes,
            Err(e) => panic!(e.to_string()),
        };
        res_vec[i] = p_sec;
    };
    for _ in 0..threads{
        let (i, p_sec) = match rx.recv(){
            Ok(mes) => mes,
            Err(e) => panic!(e.to_string()),
        };
        res_vec[i] = p_sec;
    }
    res_vec.into_iter().flat_map(|x| x).collect()
}
