fn main() {
    println!("Example p34: Release");

    /*
      cargo build --release // 生成release版本的可执行文件
      cargo build // dev

      ############################################
      cargo.toml 文件

      [profile.release]
      opt-level = 3 # 优化版本, 优化级别为3

      [profile.dev]
      opt-level = 0 # 开发版本, 不优化
      ############################################
    */
}
