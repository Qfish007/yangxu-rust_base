use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use List::{Cons, Nil};

// ==============================================
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // RefCell  目的是为了让第二个元素可以修改,RC 是为了可以拥有多个所有者
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
    parent: RefCell<Weak<Node>>, // RefCell<Weak<Node>> 目的是为了让parent可以修改
}

fn main() {
    println!("Example p42: circleAndLeak");
    // test01();
    test02();
    test03();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

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

#[allow(unused)]
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

    // let x = Rc::downgrade(&branch);

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // 将leaf的parent指向branch，但是是Weak类型
    println!("2.leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}

#[allow(unused)]
fn test03() {
    println!("--------------test03------------");

    // Rc<RefCell<T>>：多人共享一份数据，谁都能改内容
    /// 典型场景 共享可变状态（如 GUI/树节点数据）
    let a = Rc::new(RefCell::new(5));
    let b = Rc::clone(&a); // a 和 b 指向同一个 RefCell
    *b.borrow_mut() += 10;
    println!("{}", b.borrow()); // 15 —— a 的修改 b 也能看到

    /// RefCell<Rc<T>>：一个人拿着指针，能换指针指向谁
    /// 典型场景 链表/图结构里改"下一个节点指向谁"
    let m = RefCell::new(Rc::new(5));
    *m.borrow_mut() = Rc::new(6); // Rc::new(5) 被替换为 Rc::new(6)
    println!("m = {:?}", m); // Rc::new(6)` `
}
