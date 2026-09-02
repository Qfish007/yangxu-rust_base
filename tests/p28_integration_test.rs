// 集成测试
// 集成测试的内容放入tests目录下

// cargo test  // 运行所有的测试 包括集成和单元测试
// cargo test 默认会运行tests目录下的所有文件，
// 比如 p28_integration_test.rs 和 p28_integration_test2.rs
// 如果 某个文件是公共的不想运行他，将其放入tests目录下的common目录下，就不会运行了

// cargo test --test p28_integration_test // 运行指定集成测试
// cargo test --test common // 运行common目录下的所有测试

mod common;

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add_two_numbers() {
    common::set_up(); // 在测试前执行
    assert_eq!(add_two_numbers(1, 2), 3);
}
