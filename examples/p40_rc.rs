fn main() {
    println!("Example p40: rc");

    test01();
    test02();
}

fn test01() {
    use std::rc::Rc;

    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
}

fn test02() {
    println!("--------------test02------------");
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let m = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
    let a = Cons(3, Rc::clone(&m));
    let b = Cons(4, Rc::clone(&m));
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    if let Cons(x, y) = a {
        println!("{:?}", x);
        println!("{:?}", y); // Rc<List>
    }
}
