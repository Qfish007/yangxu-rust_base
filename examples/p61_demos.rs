#[derive(Clone, Debug)]
struct Persion {
    age: i32,
    name: String,
}
fn main() {
    println!("Example p0: Samples of Rust");

    let a = Persion {
        age: 18,
        name: "张三".to_string(),
    };
    let mut b = a.clone();
    b.age = 20;
    b.name = "李四".to_string();

    println!("b: {:?}", b);
    println!("a: {:?}", a);
}

// 1.实现Copy,必须同时实现Clone trait  👌🏻
// 2.实现Copy, 赋值是直接拷贝一份  👌🏻
// 3.只实现Clone, 只有调用clone方法，才是获得新的一份数据  👌🏻
// 4.实现Copy，必须所有的字段都实现Copy trait  👌🏻
