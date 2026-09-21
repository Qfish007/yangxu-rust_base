fn main() {
    println!("Example p56: 模式匹配 2");
    test01();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 15 };
    match msg {
        // @ binding 绑定
        Message::Hello { id: id_var @ 3..=7 } => println!("1. hello, id: {}", id_var),
        // id 匹配
        Message::Hello { id: 10..=12 } => println!("2. hello, id: 10..=12"),
        // 守卫
        Message::Hello { id } if id > 13 && id < 20 => println!("3. hello, id: 13..=19"),
        // 其他情况
        Message::Hello { id } => println!("4. hello, id: {}", id),
    }
}
