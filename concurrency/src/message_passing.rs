use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub(crate) fn main() {
    normal_messaging();
    multi_producing();
}

fn normal_messaging() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let value = "hi".to_string();
        tx.send(value).unwrap();
        println!("value sent");
        thread::sleep(Duration::from_secs(1));
    });

    println!("value receiving");
    println!("received: {}", rx.recv().unwrap())
}

fn multi_producing() {
    let (tx, rx) = mpsc::channel();

    // txをcloneしまくればproduceは複数作れる
    let tx1 = tx.clone();
    thread::spawn(move || {
        let v = vec![1, 2, 3, 4];
        for i in v {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let v = vec![2, 3, 4, 5];
        for i in v {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}
