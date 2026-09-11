use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    println!("Example p45_share_mutex Share Data");
    test01();
    test02();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    let m = Mutex::new(0);
    {
        let mut lock = m.lock().unwrap();
        *lock = 1;
    }
    println!("m: {:?}", m);
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut lock = counter.lock().unwrap();
            *lock += 1;
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {:?}", counter);
}
