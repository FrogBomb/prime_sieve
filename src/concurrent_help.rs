use std::thread;
use std::sync::mpsc;

#[cfg(test)]
extern crate time;

#[cfg(test)]
fn mark_time(i:usize, size: usize, s_time: time::PreciseTime){
    println!("Thread: {}, size:{} time: {}", i, size, s_time.to(time::PreciseTime::now()));
}

pub fn to_concurrent_on_section<T, SeqF>(seq_fun: SeqF,
            min_num: usize, max_num: usize, threads: usize) -> Vec<T>
            where
                T: Clone + Send + 'static,
                SeqF: Send + Sync + Copy + 'static + Fn(usize, usize) -> Vec<T>, {

    let mut res_vec:  Vec<Vec<T>> = vec![vec![]; threads];
    let seg_size = (max_num - min_num)/threads;
    let (tx, rx) = mpsc::channel();

    #[cfg(test)]
    let start = time::PreciseTime::now();

    for i in 0..threads{
        let (tx, min, max) = (tx.clone(), min_num + seg_size*i,
                                min_num + seg_size*(i+1));
        thread::spawn( move || {
            let to_send = match max-min{
                0 => vec![],
                _ => seq_fun(min, max),
            };

            #[cfg(test)]
            mark_time(i, max-min, start);

            tx.send((i, to_send)).unwrap();
        });
    }
    if (min_num + seg_size*threads) != max_num {
        res_vec.push(vec![]);
        let (tx, min, max) = (tx.clone(), min_num + seg_size*threads, max_num);
        thread::spawn( move || {
            let to_send = match max-min{
                0 => vec![],
                _ => seq_fun(min, max),
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
    res_vec.concat()
}
