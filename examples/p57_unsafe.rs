fn main() {
    println!("Example p57: Unsafe");
    // test01();
    // test02();
    // test03();
    test04();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01----解引用使用原始指针--------");

    // *const T 不可变原始指针,指针在被解引用之后，不能直接被赋值
    // *mut T 可变原始指针
    // *号是类型的一部分，不是解引用操作符

    // 原始指针的特点
    // 1. 忽略借用规则，同时存在多个可变和不可变的指针
    // 2. 不找找指向有效内存
    // 3. 可以为null
    // 4. 不会自动清理

    // * 作用，放弃安全换取性能，获得C语言的硬件交互

    let mut num = 10;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("num = {}", *r1);
        println!("num = {}", *r2);
        *r2 = 100;
    }
    println!("num = {}", num);

    let address = 0x10000;
    let p = address as *mut i32;
}

#[allow(unused)]
fn test02() {
    println!("--------------test02--------调用不安全的函数/方法--------");

    // unsafe 块
    unsafe {
        danger();
    }

    // unsafe 函数, 只能在 unsafe 块中调用
    unsafe fn danger() {
        println!("danger");
    }
}

use std::slice;
#[allow(unused)]
fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        let left = slice::from_raw_parts_mut(ptr, mid);
        let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        (left, right)
    }
}

// extern 调用外部代码 ===================================================

unsafe extern "C" {
    // ABI: Application Binary Interface, 二进制接口

    fn abs(x: i32) -> i32; // 默认是 unsafe 的
    safe fn pow(x: u32, y: u32) -> u32; // safe 承诺是安全的
}

#[allow(unused)]
fn test03() {
    println!("--------------test03--------extern 调用外部代码--------");

    /*
    FFI 外部函数调用 foreign function interface
    */

    unsafe {
        let abs = abs(-10);
        println!("abs = {}", abs);
    }

    println!("pow = {:?}", pow(1, 3)); // pow的作用是计算2的3次方
}

// 静态变量 ===================================================
static mut COUNT: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNT += inc;
    }
}

#[allow(unused)]
fn test04() {
    println!("--------------test04--------静态变量--------");

    /*
     Rust可以推断生命周期，不需要显示注释
     静态变量在内存中有固定地址，使用该值始终访问相同的数据
     常量可能在使用时复制其数据
     静态变量是可以变的，常量不可以改变
    */

    unsafe {
        add_to_count(3);
        add_to_count(2);
        // println!("COUNT = {:?}", *(&raw const COUNT));
        // println!("COUNT = {}", COUNT);
    }
    // println!("COUNT = {}", COUNT);
}

// unsafe trait ===================================================
// #[allow(unused)]
/*
# Safety
*/
// unsafe trait MyTrait {}
/*
# Safety
*/
// unsafe impl MyTrait for i32 {}

// Miri 检查器 ===================================================
#[allow(unused)]
fn test05() {
    println!("--------------test05--------Miri 检查器--------");

    // cmd
    // rustup +nightly component add miri
    // miri check
    // cargo +nightly miri run
    // cargo +nightly miri test
}
