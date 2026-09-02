fn main() {
    println!("Example p0: Samples of Rust");

    /*
     * 工作空间
     * 1. 工作空间是一个目录，用于存储项目的所有文件
     * 2. 工作空间可以包含多个项目
     * 3. 工作空间可以包含多个项目，每个项目可以包含多个 crate
     * #############################################################
     * 工作空间的目录结构如下：
     * ```
     * work_space/
     *     ├── crate1/
     *     ├── crate2/
     *     ├── crate3/
     *     ├── ...
     *     ├── Cargo.toml
     *     ├── Cargo.lock
     *     ├── README.md
     *     ├── ...
     * ```
     *
     * Cargo.toml
     *
     * [workspace]
     *
     * members = [
     *     "crate1",
     *     "crate2",
     *     "crate3",
     *     ...
     * ]
     * #############################################################
     * 如果 create1 依赖 crate2，
     * create1 的cargo.toml文件中需要添加如下内容：
     * ```
     * [dependencies]
     * crate2 = { path = "../crate2" }
     * ```
     */
}
