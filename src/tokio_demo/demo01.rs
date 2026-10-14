use futures::executor::block_on;

pub fn main() {
    // println!("--------------Tokio------------");

    // test01();
}

// 异步函数
async fn hi() {
    println!("hi");
    hello().await; // 异步函数里面可以直接调用异步函数
}

async fn hello() {
    println!("hello");
}

fn test01() {
    println!("--------------test01------------");

    // func 是一个异步函数的 Future
    let func = hi();

    // test01 是一个同步函数, 所以需要使用 block_on 来执行异步函数 hi
    block_on(func); // block_on 是一个执行器 executor
}
