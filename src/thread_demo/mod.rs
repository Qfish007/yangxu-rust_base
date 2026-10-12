#[allow(dead_code, unused)]
mod thread10_condvar;
#[allow(dead_code, unused)]
mod thread11_mpsc;
#[allow(dead_code, unused)]
mod thread12_rayon;

/*
 * 杨旭 多线程 demo
 *
 */

#[allow(dead_code, unused)]
mod thread2_builder;
#[allow(dead_code, unused)]
mod thread3_scoped;
#[allow(dead_code, unused)]
mod thread4_share_data;
#[allow(dead_code, unused)]
mod thread5_mutex_rwlock;
#[allow(dead_code, unused)]
mod thread6_atomics;
#[allow(dead_code, unused)]
mod thread7_oncecell;
#[allow(dead_code, unused)]
mod thread8_lazycell;
#[allow(dead_code, unused)]
mod thread9_parking;

// #[allow(dead_code,unused)]
// pub use thread_test as thread_demo1;
// #[allow(dead_code,unused)]
// pub use thread_scoped as thread_demo2;

pub fn main() {
    thread2_builder::main();
    thread3_scoped::main();
    thread4_share_data::main();
    thread5_mutex_rwlock::main();
    thread6_atomics::main();
    thread7_oncecell::main();
    thread8_lazycell::main();
    thread9_parking::main();
    thread10_condvar::main();
    thread11_mpsc::main();
    thread12_rayon::main();
}
