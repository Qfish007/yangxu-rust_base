use tokio::runtime;

pub fn main() {
    // println!("--------------Tokio------------");
    // test01();
    test02();
}

async fn hi() {
    println!("hello");
}

fn test01() {
    println!("--------------test01------------");

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(hi());
}

// current_thread 单线程
#[tokio::main(flavor = "current_thread")]
async fn test02() {
    // println!("--------------test02------------");
    hi().await; // cargo expand --bin rust-base 看内部实现
}

fn test03() {
    println!("--------------test03------------");

    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(5 * 1024 * 1024)
        .event_interval(20)
        .max_blocking_threads(256)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(hi());
}
