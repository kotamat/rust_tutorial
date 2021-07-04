use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) fn main() {
    mutex();
    mutex_multithreads();
}

fn mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        //  ロック中はデータが参照できない
        println!("m = {:?}", m);
        // スコープが終わるとMutexGuardが終了し、lockも解除される
    }
    println!("m = {:?}", m);
}

fn mutex_multithreads() {
    // ArcはRcのatomic版。atomicはRcに比べてコストが高い代わりに、スレッドセーフなので、マルチスレッド時のみ使う
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Arc::cloneをスレッドごとに実行し渡してあげる
        let counter = Arc::clone(&counter);
        // strong_countは並列に消化されていくので、このprintは一定にならない
        println!("Arc strong_count: {}", Arc::strong_count(&counter));
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
