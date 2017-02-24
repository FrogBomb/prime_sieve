use concurrent_help::to_concurrent_on_section;
use num_cpus;
use primal_sieve;

pub fn prime_filter_concurrently(max_num: usize, threads: usize) -> Vec<bool>{
    prime_filter_section_concurrently(0, max_num, threads)
}
pub fn prime_filter_section_concurrently(min_num:usize, max_num: usize, threads: usize) -> Vec<bool>{
    to_concurrent_on_section(prime_filter_section_sequentially, min_num, max_num, threads, 12)
}

fn int_sqrt(n:usize) -> usize{
    if n < (1 << 50) {
        (n as f64).sqrt() as usize
    }else{
        let mut x = (n as f64).sqrt() as usize;
        loop{
                x = match (x + n/x) >> 1 {
                    new_x if new_x == x => break,
                    new_x if new_x*new_x == n + 1 => {x = new_x - 1; break},
                    new_x => new_x,
                };
            }
        x
    }
}

fn ceil_sqrt(n:usize) -> usize{
    if n == 0{
        return 0;
    }
    match int_sqrt(n){
        sqrt if sqrt*sqrt == n => sqrt,
        sqrt => sqrt+1,
    }
}
pub fn prime_filter(max_num: usize) -> Vec<bool>{
    prime_filter_concurrently(max_num, num_cpus::get())
}

pub fn prime_filter_section(min_num:usize, max_num: usize) -> Vec<bool>{
    prime_filter_section_concurrently(min_num, max_num, num_cpus::get())
}

pub fn prime_filter_sequentially(max_num: usize) -> Vec<bool>{
    let ps = primal_sieve::Sieve::new(max_num);
    (0..max_num).map(|i| ps.is_prime(i)).collect()
}

pub fn prime_filter_section_sequentially(min_num:usize, max_num: usize) -> Vec<bool>{
    //Sieve of Atkin
    if min_num == 0{
        return prime_filter_sequentially(max_num);
    }
    assert!(min_num<max_num);
    let mut prime_filter = vec![false; max_num-min_num];

    set_true_if_in_range!(2, 3, 5 => prime_filter + min_num, min_num, max_num);
    //Macro equivilent:
    // if (min_num <= 2) & (max_num > 2) {prime_filter[2-min_num] = true;}
    // if (min_num <= 3) & (max_num > 3) {prime_filter[3-min_num] = true;}
    // if (min_num <= 5) & (max_num > 5) {prime_filter[5-min_num] = true;}

    let (mut y_sq, mut to_next_y_sq) = (1, 3);

    //O(sqrt(max_num)*(sqrt(max_num) - sqrt(min_num)))
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
                //While n_1<max_num,
                //When n_1 % 60 === 1, 13, 17, 29, 37, 41, 49, or 53, do
                // prime_filter[n_1-min_num] ^= true
                do_if_mod_60_match_pat!( 1 | 13 | 17 | 29 | 37 | 41 | 49 | 53, n_1 < max_num,
                    prime_filter[n_1-min_num] ^= true);
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
                //While n_2<max_num,
                //When n_2 % 60 === 7, 19, 31, or 43, do
                // prime_filter[n_2-min_num] ^= true
                do_if_mod_60_match_pat!(7 | 19 | 31 | 43, n_2 < max_num,
                    prime_filter[n_2-min_num] ^= true);
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
                //While n_3<max_num,
                //When n_3 % 60 === 11, 23, 47, or 59, do
                // prime_filter[n_3-min_num] ^= true
                do_if_mod_60_match_pat!(11 | 23 | 47 | 59, n_3 < max_num,
                    prime_filter[n_3-min_num] ^= true);
                n_3 += to_next_n_3;
                to_next_n_3 += 24;
            };
        };
        do_while!({
            y_sq += to_next_y_sq;
            to_next_y_sq += 2;
        } y_sq%6 == 0 );
    };

    //Eliminate non-squarefree numbers

    //O(max_num - min_num + sqrt(max_num))

    //This is because the inner loop
    //will iterate floor((max_num - min_num)/n^2) + 1 times for each n
    //Since the number of n's is proportional to sqrt(max_num),
    //And since the series of 1/n^2 converges, we obtain the above complexity.

    let mut n_sq = 49; // 7^2
    let mut next_n_sq = 32; //9^2 - 7^2, skip even numbers.
    while n_sq < max_num {
        let mut non_sq_free =  n_sq * match min_num/n_sq{
            k if k%2 == 1 => k,
            k => k + 1,
        };
        while non_sq_free < max_num {
            if non_sq_free >= min_num {
                prime_filter[non_sq_free - min_num] = false;
            }
            do_while!({
                non_sq_free += n_sq + n_sq;
            } (non_sq_free%3==0) | (non_sq_free%5==0));

        };
        do_while!({
            n_sq += next_n_sq;
            next_n_sq += 8;
        }(n_sq%3==0) | (n_sq%5 == 0));
    }
    prime_filter
}

#[cfg(test)]
pub fn old_prime_filter(max_num: usize) -> Vec<bool>{
    slow_prime_filter(max_num)
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
    assert_eq!(1_000_000_000, int_sqrt(10_00_000_000_000_000_000));
    assert_eq!(1<<27, ceil_sqrt((1<<54) - 1));
    assert_eq!(100000000, ceil_sqrt(9999999999989999))
}

#[cfg(test)]
fn slow_prime_filter(max_num: usize) -> Vec<bool>{
    if max_num < 5 {
         let mut ret = vec![false, false, true, true];
         ret.truncate(max_num);
         return ret
     }
    let mut prime_filter = vec![true; max_num];
    prime_filter[0] = false;
    prime_filter[1] = false;
    let mut cur_num = 2;
    'outer: loop{
        for i in (cur_num+1)..max_num{
            if 0 == i%cur_num { prime_filter[i] = false; }
        }
        cur_num += 1;
        while !prime_filter[cur_num]{
            if cur_num*cur_num > max_num {
                break 'outer
            }
            cur_num += 1;
        }
    };
    prime_filter
}
