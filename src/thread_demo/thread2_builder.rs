use std::{print, thread};

pub fn main() {
    // test01();
}

pub fn test01() {
    println!("--------------test01------------");

    let join_handle = thread::Builder::new()
        .name("test01".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(thread_event)
        .unwrap();
    join_handle.join().unwrap();
}

fn thread_event() {
    println!(
        "thread_event: thread name: {:?}, thread id: {:?}",
        thread::current().name(),
        thread::current().id()
    );
}
