fn main() {
    println!("Example p58: 高级 Traits");
    test03();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01--------高级 Traits----");
}

/*

* Self 是指实现该 trait 的类型，Rhs 是指 rhs 参数的类型，默认是 Self
 trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Adder; // Self 是 Adder 类型
impl Add for Adder {
    type Output = i32;
    fn add(self, rhs: i32) -> Self::Output {
        rhs
    }
}

*/

#[allow(unused)]
fn test02() {
    println!("--------------test02--------完全限定语法----");

    struct Dog {}
    struct Cat {}
    struct Rabbit {}
    trait Animal {
        fn name() -> &'static str {
            "Animal"
        }
    }

    impl Dog {
        fn name() -> &'static str {
            "Dog 1"
        }
    }

    impl Cat {
        fn name() -> &'static str {
            "Cat 1"
        }
    }

    impl Animal for Dog {
        fn name() -> &'static str {
            "Dog 2"
        }
    }
    impl Animal for Cat {
        fn name() -> &'static str {
            "Cat 2"
        }
    }
    impl Animal for Rabbit {}
    println!("{}", Dog::name());
    println!("{}", Cat::name());
    println!("{}", <Dog as Animal>::name()); // 输出: Dog
    println!("{}", <Cat as Animal>::name()); // 输出: Cat
    println!("{}", Rabbit::name()); // 输出: Rabbit  没有自己的方法，默认是 Animal
    println!("{}", <Rabbit as Animal>::name()); // 输出: Rabbit ，走的是 Animal 的 name 默认方法
}

#[allow(unused)]
fn test03() {
    println!("--------------test03--------supertrait----");
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = format!("{}", self);
            let len = output.len();
            // println!("{}: {}", len, output);
            println!("{}", "*".repeat(len + 4));
            println!("{}", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("{}", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point: ({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 1, y: 20000 };
    p.outline_print();
    // println!("{}", p);
}
