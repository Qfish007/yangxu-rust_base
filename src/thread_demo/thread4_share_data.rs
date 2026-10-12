// use std::println;

pub fn main() {
    // test01();
}

static DATA: [i32; 5] = [1, 2, 3, 4, 5];
static mut COUNTER: i32 = 0;
fn test01() {
    println!("--------------static------------");
    // println!("data = {:?}", DATA);

    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("data = {:?}", DATA); // 直接使用DATA, 不需要move, 因为DATA是static的，static的变量是全局的，所有线程都可以访问
            unsafe {
                COUNTER += 1; // unsafe是因为COUNTER是static mut的，static mut的变量是全局的，所有线程都可以访问，但是需要使用unsafe来访问，因为可能会有数据竞争
            }
            println!("COUNTER = {}", unsafe { COUNTER });
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn test02() {
    // println!("--------------thread_local------------");
    // thread_local! {
    //     static THREAD_LOCAL_DATA: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
    // }

    // let mut handles = Vec::new();
    // for _ in 0..10 {
    //     let handle = std::thread::spawn(|| {
    //         THREAD_LOCAL_DATA.with(|data| {
    //             *data.borrow_mut() += 1;
    //             println!("THREAD_LOCAL_DATA = {}", *data.borrow());
    //         });
    //     });
    //     handles.push(handle);
    // }
    // handles.into_iter().for_each(|h| h.join().unwrap());
}

fn test03() {
    println!("--------------Box::leak()------------");

    let data: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("data = {:?}", data);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn test04() {
    println!("--------------Arc<T>------------");

    use std::sync::{Arc, Mutex};

    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = Vec::new();
    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = std::thread::spawn(move || {
            println!("data = {:?}", data);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}
