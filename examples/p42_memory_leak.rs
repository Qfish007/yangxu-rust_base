use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use List::{Cons, Nil};

// ==============================================
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // RefCell<Rc<List>> 目的是为了让第二个元素可以修改
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }

    fn first(&self) -> Option<i32> {
        match self {
            Cons(i, _) => Some(*i),
            Nil => None,
        }
    }
}

// ==============================================
#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    test01();
    test02();
}

fn test01() {
    println!("--------------test01------------");

    println!("Example p42: circleAndLeak");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(6, RefCell::new(Rc::clone(&a)))); // 将b的第二个元素指向a
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail()); // b的第二个元素就是 RefCell::new(Rc::clone(&a))

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // 将a的第二个元素指向b
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail()); // 进入循环
    println!("a first item = {:?}", a.first());
}

fn test02() {
    println!("--------------test02------------");

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!("1.leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // 将leaf的parent指向branch，但是是Weak类型
    println!("2.leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}
