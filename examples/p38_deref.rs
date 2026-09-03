use std::ops::Deref;

fn main() {
    println!("Example p38: deref");

    test01();
    test02();
    test03();
}

fn test01() {
    let x = 5;
    let y = &x;
    println!("{:?}", y);
    println!("{:?}", *y);
}

#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn test02() {
    println!("--------------test02------------");
    let x = MyBox::new(5);
    println!("{:?}", x);
    println!("{:?}", x.deref());
    println!("{:?}", *x);
}

fn test03() {
    println!("--------------test03------------");
    let x = MyBox::new(String::from("qfish"));

    fn hello(s: &str) {
        println!("hello world :{}", s);
    }

    let str = "cbt";

    hello(str);

    hello(&x);

    // hello 要求接受一个引用类型，所以必须传递 & 类型的参数
    // &x 是 &MyBox<String> 会自动调用 deref 方法，将 x 转换为 &str
    // 1. 第一次调用 deref 方法，将 MyBox<String>  转换为 String
    // 2. 第二次调用 deref 方法，将 String 转换为 &str
    // 所以，hello(&x) 能够成功调用
}
