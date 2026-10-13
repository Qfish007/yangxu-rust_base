// use std::println;ca

use std::{
    println,
    sync::atomic::{AtomicUsize, Ordering},
};

pub fn main() {
    // test02();
}

fn test01() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..100 {
                    // let cur = counter.load(Ordering::Relaxed);
                    // counter.store(cur+1, Ordering::Relaxed);
                    counter.fetch_add(1, Ordering::Relaxed);
                    thread::sleep(std::time::Duration::from_millis(20));
                }
            });
        }

        loop {
            let n = counter.load(Ordering::Relaxed);
            if n >= 1000 {
                break;
            }

            println!("counter = {}", n);
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    println!("final counter = {}", counter.load(Ordering::Relaxed));
}

fn test02() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..100 {
                    incr(&counter);
                }
            });
        }
    });

    println!("final counter = {}", counter.load(Ordering::Relaxed));
}

fn incr(counter: &AtomicUsize) {
    let mut cur = counter.load(Ordering::Relaxed);
    loop {
        let new = cur + 1;

        // counter 和 cur比较，如果相等就把counter设置为new，否则返回当前的counter值
        match counter.compare_exchange(cur, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(x) => {
                println!("compare_exchange failed: cur = {}, x = {}", cur, x);
                cur = x;
            }
        }
    }
}
