# Day2 2021/7/3

## 学习《rust编程之道》第10章
1. 为什么要进行模块化编程？
2. Cargo包管理做了什么？
3. Cargo命令：
  - `cargo new [--bin] package [--lib]`：创建包
    - bin: 编译后生成可执行文件，默认
    - lib: 生成库文件
  - `cargo build`：编译包
  - `cargo run`：编译运行
  - `cargo test`：测试
4. Cargo.toml: 开发者维护，描述项目信息，第三方依赖
5. `#[cfg (test)]`: 条件编译: `cargo test`
6. Cargo.lock: 依赖包的详细信息，Cargo自动维护
7. Debug模式与Release模式的不同？


## 完成部分rustlings