fn main() {
    println!("Example p28: Group Test 运行与组织测试");

    // cargo test -- --test-threads=1 // 设置只有一个线程
    // cargo test -- --show-output // 显示测试输出
    // cargo test --example p28_GroupTest -- --show-output // 运行所有测试
    // cargo test --example p28_GroupTest this_test_will_pass -- --show-output // 运行指定测试
    // cargo test --example p28_GroupTest this   // 运行所有以this开头的测试

    // cargo test --example p28_GroupTest -- --ignore // 运行所有忽略的测试
    // -- --include=ignore_test // 运行指定测试，包括忽略的测试
}
#[allow(unused)]
fn prints_and_returns_10(value: i32) -> i32 {
    println!("value is: {}", value);
    10
}
#[cfg(test)]
mod tests_cbt {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(10);
        assert_eq!(value, 10);
    }

    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(10);
        assert_eq!(value, 11);
    }

    #[test]
    #[ignore] // 忽略测试
    fn this_test_will_ignore() {
        let value = prints_and_returns_10(10);
        assert_eq!(value, 11);
    }
}
