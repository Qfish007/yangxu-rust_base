use std::fs::File;
// use std::io::Read;
fn main() {
    println!("Example p23: 错误处理");

    let _v = Vec::from([1, 2, 3, 4]);

    // _v[5];

    // RUST_BACKTRACE=1 cargo run --example p23
    // RUST_BACKTRACE=full cargo run --example p23

    // 不想每次都输入 RUST_BACKTRACE=full 可以在设置
    // set RUST_BACKTRACE=full

    let file = File::open("./examples/p100_file/hello.txt");
    println!("{:?}", file);
}
