fn main() {
    println!("Example p26: 生命周期");
}
#[allow(unused)]
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    #[allow(unused)]
    fn summarize(&self) -> String {
        self.part.to_string()
    }
}

#[allow(unused)]
fn test01() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: novel.as_str(),
    };
    println!("{}", excerpt.part);
}

use std::fmt::Display;
#[allow(unused)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(unused)]
fn test02() {
    struct Foo<'a> {
        bar: &'a i32,
    }

    fn baz<'b>(f: &'b Foo) -> &'b i32 {
        f.bar
    }
}

#[allow(unused)]
fn test03() {
    struct Foo<'a> {
        bar: &'a i32,
    }

    //版本2
    fn baz2<'b>(f: &Foo<'b>) -> &'b i32 {
        f.bar
    }

    //版本5
    // fn baz5<'a, 'b>(f: &'a Foo<'b>) -> &'b i32 {
    //     f.bar
    // }
    //b 开始
    let x = 100;
    // let r1;
    // let r2;
    {
        // a 开始

        // let foo = Foo { bar: &x };
        // r1 真正指向的是 x
        // r1 = baz2(&foo);
        // r2 真正指向的是 x
        // r2 = baz5(&foo);

        // a 结束
    }
    // ✅ 这里 foo 被drop销毁！！
    // Foo这个结构体本身只是一个 “包装了引用” 的壳子。只是存放引用的容器，栈上的小对象。
    // foo里面没有拥有 i32 数据，它仅仅保存一个指针。销毁foo，只是销毁这个 “壳子”，并不会销毁它指向的 x。
    // println!("{} {}", r1, r2);
    //b 结束
}
