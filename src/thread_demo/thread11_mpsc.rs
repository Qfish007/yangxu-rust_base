use std::{sync::mpsc, thread};

pub fn main() {
    // println!("--------------MPSC------------");
    // test01();
}

type Task = Box<dyn FnOnce() + Send + 'static>;
enum Msg {
    Call(Task),
    Exit,
}

fn test01() {
    println!("--------------test01------------");

    let (tx, rx) = mpsc::channel::<Msg>();

    let handle = thread::spawn(move || {
        for msg in rx.iter() {
            match msg {
                Msg::Call(task) => task(),
                Msg::Exit => break,
            }
        }
    });

    tx.send(Msg::Call(Box::new(|| println!("hello")))).unwrap();
    tx.send(Msg::Call(Box::new(|| println!("world")))).unwrap();
    tx.send(Msg::Call(Box::new(|| println!("!")))).unwrap();
    tx.send(Msg::Exit).unwrap();

    handle.join().unwrap();
}
