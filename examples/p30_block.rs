fn main() {
    println!("Example p30: Block");
    test01();
}

#[allow(unused)]
fn test01() {
    let add_func_test01 = |a: i32, b: i32| -> i32 { a + b };
    // let add_func_test02 = |a, b| a + b;

    let add_func_test02 = |a, b| a + b;
    add_func_test02(1, 2); // 3
    add_func_test01(1, 2); // 3

    let mut opt = Vec::from([1, 2, 3]);
    // opt.sort_by(|a, b| a.cmp(b));
    opt.sort();
    println!("{:?}", opt);
}
