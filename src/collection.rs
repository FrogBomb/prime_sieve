use filter::prime_filter_section;
use std::thread;
use std::sync::mpsc;
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
    let mut res_vec: Vec<Vec<usize>> = vec![vec![]; threads];
    let seg_size = (max_num - min_num)/threads;
    let (tx, rx) = mpsc::channel();
    for i in 0..threads{
        let (tx, min, max) = (tx.clone(), min_num + seg_size*i,
                                min_num + seg_size*(i+1));
        thread::spawn( move || {
            let to_send = match min-max{
                0 => vec![],
                _ => primes_section_sequentially(min, max),
            };
            tx.send((i, to_send)).unwrap();
        });
    }
    if (min_num + seg_size*threads) != max_num {
        res_vec.push(vec![]);
        let (tx, min, max) = (tx.clone(), min_num + seg_size*threads, max_num);
        thread::spawn( move || {
            let to_send = match min-max{
                0 => vec![],
                _ => primes_section_sequentially(min, max),
            };
            tx.send((threads, to_send)).unwrap();
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
