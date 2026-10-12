use std::{
    assert_eq,
    cell::{self, Cell, RefCell},
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

fn test03() {
    println!("--------------Mutex------------");
}
