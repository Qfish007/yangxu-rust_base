use std::cell::RefCell;

fn main() {
    println!("Example p41: refcell");

    test01();
    test02();
    test03();
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Warning: You are at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You are at 75% of your quota!");
        }
    }
}

pub struct MockMessenger {
    messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> Self {
        Self {
            messages: RefCell::new(vec![]),
        }
    }
}

impl Default for MockMessenger {
    fn default() -> Self {
        Self::new()
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // 这里self是不可变引用，不能修改self，所以需要使用borrow_mut()方法获取可变引用
        self.messages.borrow_mut().push(msg.to_string());
    }
}

fn test01() {
    println!("--------------test01------------");

    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.messages.borrow().len(), 1);
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    // 等价于：
    // let mut borrow = value.borrow_mut(); // 第1步：向 RefCell 借一个 &mut i32
    // *borrow += 10; // 第2步：通过可变引用修改

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // refcell 和rc对调位置
    let value = RefCell::new(Rc::new(5));
    let mut x = value.borrow_mut();
    *x = Rc::new(6);
    println!("x = {:?}", x);
}

fn test03() {
    println!("--------------test03------------");

    {
        let v = vec![1, 2];
        let v = RefCell::new(&v);
        // v.borrow_mut().push(3); // 报错，因为v是引用，是不可变的
        println!("v = {:?}", v);
    }

    {
        let mut v = vec![1, 2];
        let v = RefCell::new(&mut v);
        v.borrow_mut().push(3); // 可以修改v
        println!("v = {:?}", v);
    }

    {
        let v = vec![1, 2];
        let v = RefCell::new(v); //
        v.borrow_mut().push(3); // 这里v是复制的，

        println!("v = {:?}", v);
    }
}
