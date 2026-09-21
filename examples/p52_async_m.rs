use std::thread;
use std::time::Duration;

use trpl::{ReceiverStream, Stream};

fn main() {
    println!("Example p52: Async Thread 综合应用");
    test01();
}

// 通过异步构建
#[allow(unused)]
fn get_intervals1() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // 启动一个异步任务，模拟发送消息
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await; // 每隔1ms发送一个消息
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

// 通过线程构建
#[allow(unused)]
fn get_intervals2() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // 启动一个线程，模拟发送消息
    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1)); // 每隔1ms发送一个消息
            count += 1;

            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

#[allow(unused)]
fn test01() {
    println!("--------------test01--------双管齐下----");

    let (tx, mut rx) = trpl::channel();

    // 使用线程发送消息到通道
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 使用异步任务接收通道中的消息
    trpl::run(async {
        while let Some(val) = rx.recv().await {
            println!("received: {}", val);
        }
    })
}
