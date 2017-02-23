use std::thread;
use std::sync::mpsc;

// #[cfg(test)]
// extern crate time;
//
// #[cfg(test)]
// fn mark_time(i:usize, bounds: (usize, usize), s_time: time::PreciseTime){
//     #[cfg(test)]
//     println!("Thread: {}, Bounds:({}, {}), time: {}", i, bounds.0, bounds.1, s_time.to(time::PreciseTime::now()));
// }

pub fn to_concurrent_on_section<T, SeqF>(seq_fun: SeqF,
            min_num: usize, max_num: usize, threads: usize, from_zero_speed_factor: usize) -> Vec<T>
            where
                T: Clone + Send + 'static,
                SeqF: Send + Sync + Copy + 'static + Fn(usize, usize) -> Vec<T>, {

    let mut res_vec:  Vec<Vec<T>> = vec![vec![]; threads];
    let (start_seg_end, seg_size) = match min_num{
        0 => {
            let seg_size = (max_num)/(threads*from_zero_speed_factor);
            (max_num - seg_size*(threads-1), seg_size)
        },
        _ => {
            let range = max_num - min_num;
            let seg_size = range/threads;
            (min_num + seg_size + range%threads, seg_size)
        },
    };

    let (tx, rx) = mpsc::channel();
    //
    // #[cfg(test)]
    // let start = time::PreciseTime::now();
    { //First section.
        let (tx, min, max) = (tx.clone(), min_num, start_seg_end);
        thread::spawn( move || {
            let to_send = match max-min{
                0 => vec![],
                _ => seq_fun(min, max),
            };
            //
            // #[cfg(test)]
            // mark_time(0, (min, max), start);
            //
            // tx.send((0, to_send)).unwrap();
        });
    }
    for i in 1..threads{
        let (tx, min, max) = (tx.clone(), start_seg_end + seg_size*(i-1),
                                start_seg_end + seg_size*i);
        thread::spawn( move || {
            let to_send = match max-min{
                0 => vec![],
                _ => seq_fun(min, max),
            };
            // 
            // #[cfg(test)]
            // mark_time(i, (min, max), start);

            tx.send((i, to_send)).unwrap();
        });
    }

    for _ in 0..threads{
        let (i, p_sec) = match rx.recv(){
            Ok(mes) => mes,
            Err(e) => panic!(e.to_string()),
        };
        res_vec[i] = p_sec;
    }
    res_vec.concat()
}
