use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

pub fn main() {
    // test01();
}

fn test01() {
    println!("--------------CondVar------------");

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        println!("step3: 子线程执行");
        let (flag, cvar) = &*pair2;
        let mut flag = flag.lock().unwrap();
        *flag = true;
        cvar.notify_one();
        println!("step4: 子线程执行完成");
    });

    let (flag, cvar) = &*pair;
    // 等待flag为true
    println!("step1: 主线程执行");
    let mut flag = flag.lock().unwrap();
    println!("step2: 主线程等待flag为true");
    while !*flag {
        flag = cvar.wait(flag).unwrap();
        println!("step5: 主线程等待flag为true,结束循环");
    }
    println!("step6: ");
}
