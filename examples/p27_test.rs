// cargo test --example p27_Test

fn main() {
    println!("Example p27: 测试");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_cbt {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic] // 测试失败
    fn test_add2() {
        // panic!("test_add2 failed"); // 测试失败
        assert_eq!(add(1, 2), 4);
    }

    #[test]
    fn test_add3() -> Result<(), String> {
        if (add(1, 2) == 3) {
            Ok(())
        } else {
            Err(String::from("test_add3 failed"))
        }
    }
}
