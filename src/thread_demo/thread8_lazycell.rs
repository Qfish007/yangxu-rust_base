use std::{cell::LazyCell, sync::LazyLock, thread};

/*
 * 都是只能赋值一次
 * LazyLock   LazyCell
 * OnceLock   OnceCell
 *
 * LazyLock   LazyCell  都是延时加载，创建的时候就要赋值
 * OnceLock   OnceCell  可以先创建，后赋值
 *
 * LazyLock  OnceLock  都是线程安全的
 * LazyCell  OnceCell  非线程安全的
 */

static NUMBER: LazyLock<u32> = LazyLock::new(|| {
    println!("NUMBER init");
    100
});
pub fn main() {
    // test02();
}

fn test01() {
    println!("--------------LazyCell------------");

    let lazy_cell = LazyCell::new(|| {
        println!("lazy_cell init");
        100
    });
    println!("step1:===========");
    println!("lazy_cell: {:?}", *lazy_cell);

    println!("step2:===========");
    println!("lazy_cell: {:?}", *lazy_cell);
}

fn test02() {
    println!("--------------LazyLock------------");
    thread::scope(|scope| {
        for _ in 0..5 {
            scope.spawn(|| {
                println!("NUMBER: {:?}", *NUMBER);
            });
        }
    });
}
