use std::thread;

fn main() {
    println!("Example p46_sync_send Send Data");
    /*
    trait Send
    所有权可以在线程之间转移，几乎所有类型都实现了 Send trait，原始指针除外

    trait Sync
    可以安全的从多个线程引用实现该 trait 的类型
    如果&T是Send,则类型T也是Sync, 即该引用可以安全的发送到另外一个线程
    (这样理解： 一般说T是Send,可以安全的发送到另外一个线程，现在&T也是Send,就说明&T也可以安全的发送到另外一个线程，所以T也是Sync)
    T: Sync 的定义就是 &T: Send。

    Send：所有权转移（move）到另一个线程安全
    Sync：把引用 &T 发到另一个线程安全（多个线程同时持有 &T）

    Send  → "能搬走"  → T 的所有权可以 move 到别的线程
    Sync  → "能借走"  → &T 可以安全地发到别的线程  T: Sync  等价于  &T: Send

    Mutex<T> 是 Sync 也是 Send
    MutexGuard<'a, T> 是 Sync 但不是 Send
    */

    test01();
    test02();
}

const X: i32 = 42;
#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    let x_ref = &X;
    let ref_x_thread = x_ref;
    let ref_x_main = x_ref;

    println!("x_ref = {}", x_ref);
    println!("ref_x_thread = {}", ref_x_thread);
    println!("ref_x_main = {}", ref_x_main);

    // X的引用可以安全的发送到线程
    let t1 = thread::spawn(move || {
        println!("in thread: {}", ref_x_thread);
    });

    println!("main thread: {}", ref_x_main);
    t1.join().unwrap();
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");
}
