use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Example p44_channels: Channels");
    // test01();
    test03();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("received: {}", received);
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
            String::from("channel"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for val in rx {
        println!("received: {}", val);
    }
}

#[allow(unused)]
fn test03() {
    println!("--------------test03------------");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("thread1 hello"),
            String::from("thread1 world"),
            String::from("thread1 rust"),
            String::from("thread1 channel"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("thread2 hello"),
            String::from("thread2 world"),
            String::from("thread2 rust"),
            String::from("thread2 channel"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for val in rx {
        println!("received: {}", val);
    }
}
