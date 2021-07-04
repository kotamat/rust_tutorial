mod message_passing;
mod shared_state;

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    // handle_spawn_thread();
    //
    // move_in_spawn_thread();
    // message_passing::main();
    shared_state::main();
}

fn move_in_spawn_thread() {
    let v = vec![1, 2, 3];
    // 明示的にvを貸すことで、spawnされたスレッドないでメインスレッドの変数を使える。
    let handle = thread::spawn(move || println!("Here's vector {:?}", v));
    // move後はメインスレッドでvは使えない
    // println!("vector is {:?}", v);
    handle.join().unwrap();
}

fn handle_spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // spawnされたスレッドを待つ
    handle.join().unwrap();
}
