use std;

macro_rules! if_any_divides {
     ( $( $div:tt ),* | $n:ident $if_b:block ) => { if $(($n % $div == 0))|* $if_b};
}
macro_rules! if_sm_prime_divides{
    ( $n:ident $if_b:block ) => {if_any_divides!(
    2, 3, 5, 7, 11, 13,
    17, 19, 23, 29, 31,
    37, 41, 43, 47, 53,
    59, 61, 67, 71, 73,
    79, 83, 89, 97, 101,
    103, 107, 109, 113, 127,
    131, 137, 139, 149, 151,
    157, 163, 167, 173, 179,
    181, 191, 193, 197, 199 | $n $if_b)};
}
macro_rules! set_true_if_in_range{
    ( $( $i:tt ),* => $filter:ident + $offset:expr, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i-$offset] = true;})*
    };
    ( $( $i:tt ),* => $filter:ident, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i] = true;})*
    };
}
macro_rules! set_sm_primes_true {
    ($filter:expr, $min:expr, $max:expr) => {
        set_true_if_in_range!(
        2, 3, 5, 7, 11, 13,
        17, 19, 23, 29, 31,
        37, 41, 43, 47, 53,
        59, 61, 67, 71, 73,
        79, 83, 89, 97, 101,
        103, 107, 109, 113, 127,
        131, 137, 139, 149, 151,
        157, 163, 167, 173, 179,
        181, 191, 193, 197, 199 => $filter + $min, $min, $max)
    };
}

pub fn prime_filter(iter_size: usize) -> Vec<bool>{
    if iter_size<100{
        slow_prime_filter(iter_size)
    }else{
        prime_filter_section(0, iter_size)
    }
}
pub fn prime_filter_section(min:usize, max: usize) -> Vec<bool>{
    //Sieve of Atkin
    assert!(min<max);
    let mut prime_filter = vec![false; max-min];

    set_true_if_in_range!(2, 3, 5 => prime_filter + min, min, max);
    //Macro equivilent:
    // if (min <= 2) & (max > 2) {prime_filter[2-min] = true;}
    // if (min <= 3) & (max > 3) {prime_filter[3-min] = true;}
    // if (min <= 5) & (max > 5) {prime_filter[5-min] = true;}



    let (mut y_sq, mut to_next_y_sq) = (1, 3);

    while y_sq<max {
        if y_sq%2 == 1 {
            //CASE 1
            //n_1 = 4x^2 + y^2 === 1 (mod 4)
            let (mut n_1, mut to_next_n_1) = (y_sq, 4);
            loop{
                n_1 += to_next_n_1;
                to_next_n_1 += 8;
                match n_1{
                    n if n < min => continue,
                    n if n >= max => break,
                    n => {match n%60{
                        1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => (),
                        _ => continue,
                    };},
                };
                prime_filter[n_1 - min] ^= true;
            };
        };
        if y_sq%3 == 1 {
            //CASE 2
            //n_2 = 3x^2 + y^2 === 1 (mod 6)
            let (mut n_2, mut to_next_n_2) = (y_sq, 3);
            loop {
                n_2 += to_next_n_2;
                to_next_n_2 += 6;
                match n_2{
                    n if n < min => continue,
                    n if n >= max => break,
                    n => {match n%60{
                            7 | 19 | 31 | 43 => (),
                            _ => continue,
                        };}
                };

                prime_filter[n_2 - min] ^= true;
            };
            //CASE 3
            //n_3 = 3x^2 - y^2 === 11 (mod 12)
            //Initially, we set x = y+1 -> n_3 = 3(y+1)^2 - y^2 = 2*y^2 + 6*y + 3
            //Amd then hop x by 2 each iteration.
            let (mut n_3, mut to_next_n_3) = (2*y_sq+3*to_next_y_sq, 6*to_next_y_sq+18);
            loop {
                match n_3{
                    n if n < min => (),
                    n if n >= max => break,
                    n => {match n%60{
                            11 | 23 | 47 | 59 => {prime_filter[n_3 - min] ^= true;},
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
