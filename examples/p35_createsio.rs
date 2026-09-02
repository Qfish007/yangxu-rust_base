//! 代码解释标题 测试doc文件注释
//!
//! 测试doc代码解释内容1 测试doc
//!
//! 测试doc 代码解释内容2
//!

/// Adds two numbers together by cbt
///
/// # Example1
/// ```
/// let result = add(1, 2);
/// assert_eq!(result, 3);
/// ```
///
///
//
// 这样写方便在其他模块中使用 kinds 中的枚举类型
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// cargo doc --example p35_createsio
// cargo doc --open --example p35_createsio

// 注意这段需要在lib.rs中才能运行注释Example1 内部的函数
// cargo test

pub mod kinds {

    pub enum PrimaryColor {
        Red,
        Green,
        Blue,
        Yellow,
        Orange,
        Purple,
    }

    pub enum SecondaryColor {
        Orange,
        Purple,
        Brown,
        Grey,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

fn main() {
    println!("Example p35: creates.io");

    /*
     * 如何使用 creates.io
     * step1: 获取token
     * step2: cargo login --registry creates.io
     * step3: 输入 token
     * step4: cargo.toml 输入必要的内容
     * step5: 发布 cargo publish
     * step6: 废弃版本 cargo yank --vers 1.0.0
     * step7: 撤销废弃版本 cargo yank --vers 1.0.0 --undo
     */
}
