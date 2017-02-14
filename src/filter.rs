extern crate num_cpus;

use std;
use std::thread;
use std::sync::mpsc;

pub fn prime_filter(iter_size: usize) -> Vec<bool>{
    if iter_size<100{
        slow_prime_filter(iter_size)
    }else{
        prime_filter_section(0, iter_size)
    }
}

fn case_1(y_sq: usize, iter_size: usize)
    -> Vec<usize> {
    //n_1 = 4x^2 + y^2 === 1 (mod 4)
    let (mut n_1, mut to_next_n_1) = (y_sq, 4);    let mut ret_stack: Vec<usize> = Vec::new();
    loop{
        n_1 += to_next_n_1;
        to_next_n_1 += 8;
        match n_1%60{
            _ if n_1 >= iter_size => break,
            1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => (),
            _ => continue,
        };

        // println!("1: {}", n_1);
        ret_stack.push(n_1);
    };
    ret_stack
}
fn case_2(y_sq: usize, iter_size: usize)
    -> Vec<usize> {
    //n_2 = 3x^2 + y^2 === 1 (mod 6)
    let (mut n_2, mut to_next_n_2) = (y_sq, 3);
    let mut ret_stack: Vec<usize> = Vec::new();
    loop {
        n_2 += to_next_n_2;
        to_next_n_2 += 6;
        match n_2%60{
            _ if n_2 >= iter_size => break,
            7 | 19 | 31 | 43 => (),
            _ => continue,
        };
        // println!("2: {}", n_2);
        ret_stack.push(n_2);
    };
    ret_stack
}
fn case_3(y_sq: usize, to_next_y_sq: usize, iter_size: usize)
    -> Vec<usize> {
    //n_3 = 3x^2 - y^2 === 11 (mod 12)
    let (mut n_3, mut to_next_n_3) = (2*y_sq, 3*to_next_y_sq);
    let mut ret_stack: Vec<usize> = Vec::new();
    loop {
        n_3 += to_next_n_3;
        to_next_n_3 += 6;
        match n_3%60{
            _ if n_3 >= iter_size => break,
            11 | 23 | 47 | 59 => (),
            _ => continue,
        };
        // println!("3: {}", n_3);
        ret_stack.push(n_3);
    };
    ret_stack
}

fn prime_filter_section(min:usize, max: usize) -> Vec<bool>{
    //Sieve of Atkin
    let num_cpus = num_cpus::get();
    assert!(min<max);
    let mut prime_filter = vec![false; max-min];
    if (min <= 2) & (max > 2) {prime_filter[2-min] = true;}
    if (min <= 3) & (max > 3) {prime_filter[3-min] = true;}
    if (min <= 5) & (max > 5) {prime_filter[5-min] = true;}

    let (mut y_sq, mut to_next_y_sq) = match min {
        0|1 => (1, 3),
        _ => (min*min, 2*min + 1),
    };
    let mut spawned_threads = 0;
    let (tx, rx) = mpsc::channel();
    while y_sq<max {
        if y_sq%2 == 1 {
            spawned_threads += 1;
            let (y_sq, tx) = (y_sq, tx.clone());
            thread::spawn(move || {
                tx.send(case_1(y_sq, max)).unwrap();
            });
        };
        if y_sq%3 == 1 {

            spawned_threads += 1;
            let (y_sq, to_next_y_sq, tx1) = (y_sq, to_next_y_sq, tx.clone());
            thread::spawn(move || {
                tx1.send(case_2(y_sq, max)).unwrap();
            });
            if y_sq*2<max{
                spawned_threads += 1;
                let tx2 = tx.clone();
                thread::spawn(move || {
                    tx2.send(case_3(y_sq, to_next_y_sq, max)).unwrap();
                });
            }
        };
        while{ //Do-while
            y_sq += to_next_y_sq;
            to_next_y_sq += 2;
            y_sq%6 == 0
        } {};
        while (spawned_threads + 1)>=num_cpus{
            for mes in rx.try_iter(){
                spawned_threads -= 1;
                for flip_i in mes{
                    prime_filter[flip_i - min] ^= true;
                }
            }
        };
    };

    while spawned_threads!=0{
        for mes in rx.try_iter(){
            spawned_threads -= 1;
            for flip_i in mes{
                prime_filter[flip_i - min] ^= true;
            }
        }
    };
    //Eliminate non-squarefree numbers
    let mut n_sq = 49; // 7^2
    let mut next_n_sq = 32; //9^2 - 7^2, skip even numbers.
    while n_sq < max {
        let mut non_sq_free = n_sq;
        while non_sq_free < max {
            if non_sq_free >= min {
                prime_filter[non_sq_free - min] = false;
            }
            while{ //Do-while
                non_sq_free += n_sq + n_sq;
                (non_sq_free%3==0) | (non_sq_free%5==0)
            } {};
        };
        while{ //Do-while
            n_sq += next_n_sq;
            next_n_sq += 8;
            (n_sq%3==0) | (n_sq%5 == 0)
        } {};
    }
    prime_filter
}
#[cfg(test)]
pub fn old_prime_filter(iter_size: usize) -> std::vec::Vec<bool>{
    slow_prime_filter(iter_size)
}

fn slow_prime_filter(iter_size: usize) -> std::vec::Vec<bool>{
    if iter_size < 5 {
         let mut ret = vec![false, false, true, true];
         ret.truncate(iter_size);
         return ret
     }
    let mut prime_filter = vec![true; iter_size];
    prime_filter[0] = false;
    prime_filter[1] = false;
    let mut cur_num = 2;
    'outer: loop{
        for i in (cur_num+1)..iter_size{
            if 0 == i%cur_num { prime_filter[i] = false; }
        }
        cur_num += 1;
        while !prime_filter[cur_num]{
            if cur_num*cur_num > iter_size {
                break 'outer
            }
            cur_num += 1;
        }
    };
    prime_filter
}
