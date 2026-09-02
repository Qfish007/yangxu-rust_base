/*
examples 使用文件夹，需要在 Cargo.toml 中配置
[[example]]
name = "p100"
path = "examples/p100_file/p100.rs"

*/

use std::fs::File;
// use std::io::Read;
fn main() {
    println!("Example p100: Hello, world!");

    let _v = Vec::from([1, 2, 3, 4]);

    // _v[5];

    // RUST_BACKTRACE=1 cargo run --example p23
    // RUST_BACKTRACE=full cargo run --example p23

    // 不想每次都输入 RUST_BACKTRACE=full 可以在设置
    // set RUST_BACKTRACE=full

    let file = File::open("./examples/p100_file/hello.txt");
    println!("{:?}", file);
}
