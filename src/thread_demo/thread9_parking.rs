use std::{
    print,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub fn main() {
    // test01();
}

fn test01() {
    println!("--------------Parking------------");

    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = flag.clone();

    let parked_thread = thread::spawn(move || {
        // 等待flag为true
        while !flag2.load(Ordering::Relaxed) {
            println!("step1: parkarked_thread sleep");
            thread::park(); // 线程挂起了
            println!("step2: parkarked_thread wake up");
        }
        println!("step3: parkarked_thread exit");
    });
    thread::sleep(Duration::from_millis(1000)); // 主线程等待1秒,
    flag.store(true, Ordering::Relaxed); // 主线程将flag设置为true
    parked_thread.thread().unpark(); // 主线程唤醒parkarked_thread
    parked_thread.join().unwrap(); // 等待parkarked_thread执行完成
}
