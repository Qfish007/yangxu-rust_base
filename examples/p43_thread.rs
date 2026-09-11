use std::thread;
use std::time::Duration;
fn main() {
    println!("Example p43: Thread");
    test01();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    thread::spawn(|| {
        for i in 0..10 {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 0..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
