use std;
use std::thread;
use std::sync::mpsc;

fn case_1(y_sq: usize, iter_size: usize)
    -> (usize, Vec<bool>) {
    //n_1 = 4x^2 + y^2 === 1 (mod 4)
    let (mut n_1, mut to_next_n_1) = (y_sq, 4);
    let offset = n_1;
    let mut temp_filter = vec![false; iter_size-offset];
    loop{
        n_1 += to_next_n_1;
        to_next_n_1 += 8;
        match n_1%60{
            _ if n_1 >= iter_size => break,
            1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => (),
            _ => continue,
        };

        // println!("1: {}", n_1);
        temp_filter[n_1-offset] ^= true;
    };
    (offset, temp_filter)
}
fn case_2(y_sq: usize, iter_size: usize)
    -> (usize, Vec<bool>) {
    //n_2 = 3x^2 + y^2 === 1 (mod 6)
    let (mut n_2, mut to_next_n_2) = (y_sq, 3);
    let offset = n_2.clone();

    let mut temp_filter = vec![false; iter_size-offset];
    loop {
        n_2 += to_next_n_2;
        to_next_n_2 += 6;
        match n_2%60{
            _ if n_2 >= iter_size => break,
            7 | 19 | 31 | 43 => (),
            _ => continue,
        };
        // println!("2: {}", n_2);
        temp_filter[n_2 - offset] ^= true;
    };
    (offset, temp_filter)
}
fn case_3(y_sq: usize, to_next_y_sq: usize, iter_size: usize)
    -> (usize, Vec<bool>) {
    //n_3 = 3x^2 - y^2 === 11 (mod 12)
    let (mut n_3, mut to_next_n_3) = (2*y_sq, 3*to_next_y_sq);
    let offset = n_3;
    let mut temp_filter = vec![false; iter_size - offset];
    loop {
        n_3 += to_next_n_3;
        to_next_n_3 += 6;
        match n_3%60{
            _ if n_3 >= iter_size => break,
            11 | 23 | 47 | 59 => (),
            _ => continue,
        };
        // println!("3: {}", n_3);
        temp_filter[n_3 - offset] ^= true;
    };
    (offset, temp_filter)
}

pub fn prime_filter(iter_size: usize) -> Vec<bool>{
    //Sieve of Atkin
    if iter_size < 100 {
        slow_prime_filter(iter_size)
    }else{
        let mut prime_filter = vec![false; iter_size];
        prime_filter[2] = true;
        prime_filter[3] = true;
        prime_filter[5] = true;
        let mut y_sq = 1;
        let mut to_next_y_sq = 3;

        let mut spawned_threads = 0;

        let (tx, rx) = mpsc::channel();
        while y_sq<iter_size {
            if y_sq%2 == 1 {
                spawned_threads += 1;
                let (y_sq, tx) = (y_sq, tx.clone());
                thread::spawn(move || {
                    tx.send(case_1(y_sq, iter_size)).unwrap();
                });
            };
            if y_sq%3 == 1 {

                spawned_threads += 1;
                let (y_sq, to_next_y_sq, tx1) = (y_sq, to_next_y_sq, tx.clone());
                thread::spawn(move || {
                    tx1.send(case_2(y_sq, iter_size)).unwrap();
                });
                if y_sq*2<iter_size{
                    spawned_threads += 1;
                    let tx2 = tx.clone();
                    thread::spawn(move || {
                        tx2.send(case_3(y_sq, to_next_y_sq, iter_size)).unwrap();
                    });
                }
            };
            if spawned_threads>0{
                for mes in rx.try_iter(){
                    spawned_threads -= 1;
                    let (offset, temp_filter) = mes;
                    for flip_i in temp_filter.into_iter().enumerate()
                                            .filter_map(|x| match x {
                                                (i, true) => Some(i),
                                                _ => None,
                                            }){
                        prime_filter[offset + flip_i] ^= true;
                    }
                }
            }
            while{ //Do-while
                y_sq += to_next_y_sq;
                to_next_y_sq += 2;
                y_sq%6 == 0
            } {};
        };
        while spawned_threads>0{
            for mes in rx.try_iter(){
                spawned_threads -= 1;
                let (offset, temp_filter) = mes;
                for flip_i in temp_filter.into_iter().enumerate()
                                        .filter_map(|x| match x {
                                            (i, true) => Some(i),
                                            _ => None,
                                        }){
                    prime_filter[offset + flip_i] ^= true;
                }
            }
        }
        //Eliminate non-squarefree numbers
        let mut n_sq = 49; // 7^2
        let mut next_n_sq = 32; //9^2 - 7^2, skip even numbers.
        while n_sq < iter_size {
            let mut non_sq_free = n_sq;
            while non_sq_free < iter_size {
                prime_filter[non_sq_free] = false;
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
