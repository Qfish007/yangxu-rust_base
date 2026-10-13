use core::panic;
use std::{
    assert_eq,
    cell::{self, Cell, RefCell},
    println,
    sync::{Arc, Mutex},
    thread,
};

pub fn main() {}

fn test01() {
    println!("--------------cell<T>------------");

    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5); // get 需要 Copy trait, 5是Copy的，所以可以直接get
    assert_eq!(cell.replace(10), 5); // replace 返回的是旧值5
    assert_eq!(cell.get(), 10);

    let ten = cell.into_inner(); // into_inner() 返回的是旧值10, cell被销毁了(取出了值，cell就没有存在的意义了)

    let cell = Cell::new(String::from("hello"));
    assert_eq!(cell.take(), "hello"); // 取出了hello, cell里面的值变成了String::default()，也就是空字符串
    assert_eq!(cell.take(), String::default());

    cell.set(String::from("world"));
    // cell.get(); // get 需要 Copy trait, String没有实现Copy trait，所以不能直接get
}

fn test02() {
    println!("--------------RefCell<T>------------");

    let ref_cell = RefCell::new(5);
    assert_eq!(*ref_cell.borrow(), 5); // borrow 返回的是 Ref<T>，需要解引用才能得到值
    *ref_cell.borrow_mut() = 10; // borrow_mut 返回的是 RefMut<T>，需要解引用才能修改值
    assert_eq!(*ref_cell.borrow(), 10);

    let mut mut_ref = ref_cell.borrow_mut();
    *mut_ref = 20;
    assert_eq!(*ref_cell.borrow(), 20);

    // let mut_ref2 = ref_cell.borrow_mut(); // panic! 因为已经有一个可变借用存在了，不能再借用一个可变借用
}

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
fn test03() {
    println!("--------------Mutex------------");

    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = std::thread::spawn(move || {
            let mut numbers = NUMBERS.lock().unwrap();
            numbers.push(i);
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("numbers = {:?}", *lock);
}

fn test04() {
    println!("--------------Mutex + poisoned ------------");

    let data = Arc::new(Mutex::new(0));
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut lock = data.lock().unwrap();
            *lock += 1;
            panic!(); // 这里会导致锁被poisoned，其他线程再去lock的时候会返回Err
        })
        .join()
        .unwrap_err();
    }
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            match data.lock() {
                Ok(mut lock) => {
                    *lock += 1;
                }
                Err(poisoned) => {
                    println!("poisoned: {:?}", poisoned);
                    let mut lock = poisoned.into_inner(); // 这里会返回锁的内部数据，继续使用
                    *lock += 1;
                    println!("lock after poisoned: {:?}", *lock);
                }
            }
        })
        .join()
        .unwrap();
    }
}

fn test05() {
    println!("--------------RwLock------------");

    use std::sync::RwLock;

    let rw_lock = RwLock::new(5);

    {
        let r1 = rw_lock.read().unwrap();
        let r2 = rw_lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = rw_lock.write().unwrap();
        *w += 1;
    }

    {
        let r = rw_lock.read().unwrap();
        assert_eq!(*r, 6);
    }
}
