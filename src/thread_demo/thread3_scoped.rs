use std::time::Duration;
use std::{println, thread};

pub fn main() {
    // test02();
    // test03();
}

fn test01() {
    println!("--------------test01------------");

    let mut handles = Vec::new();

    for i in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("thread {} is running", i);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn test02() {
    println!("--------------test02------------");

    let name = "test02".to_string();
    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(|| {
                thread::sleep(Duration::from_millis(100));
                println!("thread {} is running", name); // name 可以直接使用，
            });
        }
    });

    // 注意：变量在 thread::scope 上面的可以直接使用，下面的就不行了。内部的也不行
    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                thread::sleep(Duration::from_millis(100));
                println!("thread {} is running", i); // i 需要move，
            });
        }
    });
}

fn test03() {
    println!("--------------test03------------");

    const SIZE: usize = 10;
    let numbers = (1..10000).collect::<Vec<_>>();
    let chunks = numbers.chunks(SIZE);

    let sum = thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in chunks {
            let handle = s.spawn(move || chunk.iter().sum::<u64>());
            handles.push(handle);
        }

        handles.into_iter().map(|h| h.join().unwrap()).sum::<u64>()
    });

    println!("sum = {}", sum);
}
