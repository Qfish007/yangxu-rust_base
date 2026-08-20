use core::panic;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::num::ParseIntError;
fn main() {
    println!("Example p23: 错误处理");

    let _v = Vec::from([1, 2, 3, 4]);

    // _v[5];

    // RUST_BACKTRACE=1 cargo run --example p23
    // RUST_BACKTRACE=full cargo run --example p23

    // 不想每次都输入 RUST_BACKTRACE=full 可以在设置
    // set RUST_BACKTRACE=full

    test01();
    test02();
}

// 错误传播
fn read_username_from_file() -> Result<String, Error> {
    // 这里读文件可能产生错误，所以要处理错误
    let file = File::open("./examples/static/file/p23_hello.txt");
    let mut file2 = file?;

    // 这里读取文件内容可能产生错误，所以要处理错误
    let mut content = String::new();

    match file2.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e), // 往上面抛
    }
}

#[allow(unused)]
fn read_username_from_file_simple() -> Result<String, Error> {
    // method1
    // let mut content = String::new();
    // File::open("./examples/static/file/p23_hello.txt")?.read_to_string(&mut content)?;
    // Ok(content)

    // method2
    fs::read_to_string("./examples/static/file/p23_hello.txt")
}

fn test01() {
    let file = File::open("./examples/static/file/p23_hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            // 文件不存在，创建文件
            std::io::ErrorKind::NotFound => {
                match File::create("./examples/static/file/p23_hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Error: {:?}", e),
                }
            }
            // 其他错误
            _ => panic!("Error: {:?}", e),
        },
    };
    println!("{:?}", file);
}

fn test02() {
    // ========================================================================================
    let file2 = File::open("./examples/static/file/none.txt").expect("😭cbt File not found😭");
    println!("{:?}", file2);

    let result = read_username_from_file();
    let content = match result {
        Ok(content) => content,
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("{:?}", content);
}

#[allow(unused)]
fn test03() {
    #[derive(Debug)]
    pub enum MyError {
        Io(Error),
        ParseInt(ParseIntError),
        Other(String),
    }

    // 实现 From trait，将 Error 转换为 MyError
    impl From<Error> for MyError {
        fn from(e: Error) -> Self {
            Self::Io(e)
        }
    }

    // 实现 From trait，将 ParseIntError 转换为 MyError
    impl From<ParseIntError> for MyError {
        fn from(e: ParseIntError) -> Self {
            Self::ParseInt(e)
        }
    }

    fn read_username_from_file() -> Result<String, MyError> {
        let mut name = String::new();
        let file = File::open("./examples/static/file/p23_hello.txt")?.read_to_string(&mut name)?;
        let num = "55".parse::<i32>()?; // 解析失败会通过 From<ParseIntError> for MyError 转换为 MyError
        println!("num: {}", num);
        Ok(name)
    }

    #[allow(unused)]
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
