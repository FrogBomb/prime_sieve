use std;
use std::thread;
use std::sync::mpsc;
use num_cpus;
// macro_rules! if_any_divides {
//      ( $( $div:tt ),* | $n:ident $if_b:block ) => { if $(($n % $div == 0))|* $if_b};
// }
// macro_rules! if_sm_prime_divides{
//     ( $n:ident $if_b:block ) => {if_any_divides!(
//     2, 3, 5, 7, 11, 13,
//     17, 19, 23, 29, 31,
//     37, 41, 43, 47, 53,
//     59, 61, 67, 71, 73,
//     79, 83, 89, 97, 101,
//     103, 107, 109, 113, 127,
//     131, 137, 139, 149, 151,
//     157, 163, 167, 173, 179,
//     181, 191, 193, 197, 199 | $n $if_b)};
// }
macro_rules! set_true_if_in_range{
    ( $( $i:tt ),* => $filter:ident + $offset:expr, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i-$offset] = true;})*
    };
    ( $( $i:tt ),* => $filter:ident, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i] = true;})*
    };
}
// macro_rules! set_sm_primes_true {
//     ($filter:expr, $min:expr, $max:expr) => {
//         set_true_if_in_range!(
//         2, 3, 5, 7, 11, 13,
//         17, 19, 23, 29, 31,
//         37, 41, 43, 47, 53,
//         59, 61, 67, 71, 73,
//         79, 83, 89, 97, 101,
//         103, 107, 109, 113, 127,
//         131, 137, 139, 149, 151,
//         157, 163, 167, 173, 179,
//         181, 191, 193, 197, 199 => $filter + $min, $min, $max)
//     };
// }
pub fn prime_filter_concurrently(iter_size: usize, threads: usize) -> Vec<bool>{
    prime_filter_section_concurrently(0, iter_size, threads)
}
pub fn prime_filter_section_concurrently(min_num:usize, max_num: usize, threads: usize) -> Vec<bool>{
    let mut res_vec: Vec<Vec<bool>> = vec![vec![]; threads];
    let seg_size = (max_num - min_num)/threads;
    let (tx, rx) = mpsc::channel();
    for i in 0..threads{
        let (tx, min, max) = (tx.clone(), min_num + seg_size*i,
                                min_num + seg_size*(i+1));
        thread::spawn( move || {
            tx.send((i, prime_filter_section_sequentially(min, max))).unwrap();
        });
    }
    if (min_num + seg_size*threads) != max_num {
        let (tx, min, max) = (tx.clone(), min_num + seg_size*threads, max_num);
        thread::spawn( move || {
            tx.send((threads, prime_filter_section_sequentially(min, max))).unwrap();
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

fn int_sqrt(n:usize) -> usize{
    match n {
        0 => 0,
        1 ... 3 => 1,
        4 ... 8 => 2,
        9 ... 15 => 3,
        k => {
        let mut x = k;
            loop{
                x = match (x + n/x) >> 1 {
                    new_x if new_x == x => break,
                    new_x if new_x*new_x == n + 1 => {x = new_x - 1; break},
                    new_x => new_x,
                };
            }
            x
        },
    }

}

fn ceil_sqrt(n:usize) -> usize{
    match int_sqrt(n){
        sqrt if sqrt*sqrt == n => sqrt,
        sqrt => sqrt+1,
    }
}
pub fn prime_filter(iter_size: usize) -> Vec<bool>{
    prime_filter_sequentially(iter_size)
}

pub fn prime_filter_section(min_num:usize, max_num: usize) -> Vec<bool>{
    prime_filter_section_sequentially(min_num, max_num)
}

pub fn prime_filter_sequentially(iter_size: usize) -> Vec<bool>{
    if iter_size<100{
        slow_prime_filter(iter_size)
    }else{
        prime_filter_section(0, iter_size)
    }
}
pub fn prime_filter_section_sequentially(min_num:usize, max_num: usize) -> Vec<bool>{
    //Sieve of Atkin
    assert!(min_num<max_num);
    let mut prime_filter = vec![false; max_num-min_num];

    set_true_if_in_range!(2, 3, 5 => prime_filter + min_num, min_num, max_num);
    //Macro equivilent:
    // if (min_num <= 2) & (max_num > 2) {prime_filter[2-min_num] = true;}
    // if (min_num <= 3) & (max_num > 3) {prime_filter[3-min_num] = true;}
    // if (min_num <= 5) & (max_num > 5) {prime_filter[5-min_num] = true;}



    let (mut y_sq, mut to_next_y_sq) = (1, 3);

    while y_sq<max_num {
        if y_sq%2 == 1 {
            //CASE 1
            //n_1 = 4x^2 + y^2 === 1 (mod 4)
            let (mut n_1, mut to_next_n_1) = match y_sq < min_num {
                false => (y_sq+4, 12),
                _ => {
                    let min_num_x = (ceil_sqrt(min_num - y_sq) +1)/2;
                    (4*min_num_x*min_num_x + y_sq, 8*min_num_x + 4)
                },
            };
            loop{

                match n_1{
                    n if n >= max_num => break,
                    n => {match n%60{
                        1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => prime_filter[n-min_num] ^= true,
                        _ => (),
                    };},
                };

                n_1 += to_next_n_1;
                to_next_n_1 += 8;
            };
        };
        if y_sq%3 == 1 {
            //CASE 2
            //n_2 = 3x^2 + y^2 === 1 (mod 6)
            let (mut n_2, mut to_next_n_2) = match y_sq < min_num {
                false => (y_sq+3, 9),
                _ => {
                    let min_num_x = (ceil_sqrt((min_num - y_sq)*3)+2)/3;
                    (3*min_num_x*min_num_x + y_sq, 6*min_num_x + 3)
                },
            };
            loop {
                match n_2{
                    n if n >= max_num => break,
                    n => {match n%60{
                            7 | 19 | 31 | 43 => prime_filter[n-min_num] ^= true,
                            _ => (),
                        };}
                };
                n_2 += to_next_n_2;
                to_next_n_2 += 6;
            };
            //CASE 3
            //n_3 = 3x^2 - y^2 === 11 (mod 12)
            //Initially, we set x = y+1 -> n_3 = 3(y+1)^2 - y^2 = 2*y^2 + 6*y + 3
            //Amd then hop x by 2 each iteration.
            let (mut n_3, mut to_next_n_3) = match (y_sq << 1) < min_num {
                false => (2*y_sq+3*to_next_y_sq, 6*to_next_y_sq+18),
                _ => {
                    let min_num_x = match (ceil_sqrt((min_num + y_sq)*3) +2)/3{
                        mx if (mx+y_sq)%2 == 0 => mx + 1,
                        mx => mx,
                    };
                    (3*min_num_x*min_num_x - y_sq, 12*min_num_x + 12)
                },
            };
            loop {
                match n_3{
                    n if n >= max_num => break,
                    n => {match n%60{
                            11 | 23 | 47 | 59 => prime_filter[n-min_num] ^= true,
                            _ => (),
                    };},
                };
                n_3 += to_next_n_3;
                to_next_n_3 += 24;
            };
        };
        while{ //Do-while
            y_sq += to_next_y_sq;
            to_next_y_sq += 2;
            y_sq%6 == 0
        } {};
    };

    //Elimin_numate non-squarefree numbers
    let mut n_sq = 49; // 7^2
    let mut next_n_sq = 32; //9^2 - 7^2, skip even numbers.
    while n_sq < max_num {
        let mut non_sq_free = n_sq;
        while non_sq_free < max_num {
            if non_sq_free >= min_num {
                prime_filter[non_sq_free - min_num] = false;
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

#[test]
fn private_filter_test(){
    assert_eq!(5, ceil_sqrt(24));
    assert_eq!(2, int_sqrt(4));
    assert_eq!(4, int_sqrt(24));
    assert_eq!(10, int_sqrt(101));
    assert_eq!(1, int_sqrt(1));
    assert_eq!(10, int_sqrt(100));
    assert_eq!(3, int_sqrt(13));
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
