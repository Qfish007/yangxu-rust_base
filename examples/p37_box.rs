fn main() {
    println!("Example p37: box");

    let mut b = Box::new(5);
    *b = 10;
    println!("b = {}", b);

    test01();
}

use List::Cons;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn test01() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    if let Cons(x, y) = list {
        println!("{:?}", x);
        println!("{:?}", y); // Box<List>
    }
}
