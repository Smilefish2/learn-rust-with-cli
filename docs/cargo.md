# 初始化一个项目
```
# 默认创建src/main.rs模式的应用程序项目
cargo init learn-rust-with-cli
# 使用--lib参数创建类库类型项目
cargo init rust-by-example --lib
```
# 运行项目
```
cargo run
```
# 构建项目
```
// debug
// -v = --verbose
cargo build -v
// release
cargo build --release
```
# 清理项目
```
// -v = --verbose
cargo clean -v
```
# 更新/升级依赖
```
cargo update
cargo update -p <包名>
```
# 检查项目(快速检查语法，等于快速编译，可用于提升开发速度)
```
cargo check
```
# 单元测试
```
cargo test
```
# 创建代码库项目
```
cargo new --lib <包名>
```
# 常用包依赖语法形式（https://crates.io/）
```
[dependencies]
clap = "~2.33.3"
[dependencies.clap]
git = "https://github.com/clap-rs/clap.git"
```