fn main() {
    println!("Example p39: drop");

    test01();
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");
    let csp = CustomSmartPointer {
        data: String::from("qfish"),
    };

    drop(csp); // 手动调用drop函数，会提前释放资源

    println!("--------------test01 end------------");
}
