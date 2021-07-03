<!--
 * @Author: Sakura
 * @Date: 2021-07-03 08:28:26
 * @LastEditTime: 2021-07-03 23:52:52
 * @Description: 第二天的学习
-->
# Day2 2021/7/3

## 学习《rust编程之道》第9章
- 非正常情况
  - 失败：不满足程序正确运行条件（契约）（编译时+运行时） => 类型系统、断言
  - 错误：意料之中的问题，且可以有解决方案(查询数据返回空，打开不存在的文件)
  - 异常：完全不可预料的问题（空指针...)
- 失败
  - 强大的类型系统
    - 不允许隐式转换
  - 断言
    - assert!
    - assert_eq!
    - assert_ne!
    - debug模式
    - 快速失败策略：尽早暴露错误
    - 契约： 前置条件、后置条件、前后不变

### 分层处理错误
- `Option<T>: Some | None`：对None值做特殊处理
- `Result<T, E>: OK | Err`：处理可以合理解决的问题
- `Panic`：线程恐慌，处理无法解决的问题
- `Abort`：中止，处理会发生灾难性后果的情况

#### Option<T>
- 应用场景： 可选或者可空的情况，主要消除空指针问题
- unwrap方法：取出Some中的值
  - `unwrap(): None => Panic!()` (为了不造成线程恐慌就需要预先判断是否为`None`(`is_some()`)，这样就造成两次判断（`unwrap()`)
  - `unwrap_or(): match匹配的语法糖`
  - `unwrap_or_else(): 类似_or`
- `expect`: None => Panic!(&str)`
- `map[_or[_else]]<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U>`: match匹配的包装，无须取出值，直接在Option<T>内部计算（组合子Combinator）
- 由于`map()`方法返回`U`类型，如果该闭包刚好返回Option<T>则会造成Some的多层嵌套，所以有另外的组合子`and_then() -> Option<U>, U: f(x), F -> Option<U>`

#### Result<T, E>
- 应用场景：错误处理
- Result<T, ()> <=> Option<T>
- 程序员必须对Result<T, E>的结果进行处理
- 

## 学习《rust编程之道》第10章
### Cargo

-  Cargo包管理做了什么？
   -  两个元数据文件记录项目信息
   -  获取并构建项目依赖关系
   -  构建项目
   -  统一的工作流
   -  便捷的管理包
-  Cargo命令：
 - `cargo new [--bin] package [--lib]`：创建包
   - bin: 编译后生成可执行文件，默认
   - lib: 生成库文件
 -  `cargo build`：编译包
 - `cargo run`：编译运行
 - `cargo test`：测试
 - `cargo fix`: 自动修复Warning
 - `cargo fmt`, `cargo clippy`, 自定义Cargo子命令
-  Cargo.toml: 开发者维护，描述项目信息，第三方依赖
-  `#[cfg (test)]`: 条件编译: `cargo test`
-  Cargo.lock: 依赖包的详细信息，Cargo自动维护
-  Debug模式与Release模式的不同？
   -  Release模式会对代码进行优化，编译变慢但运行更快
-  Constant与Static有什么异同？
   -  Constant与Static都是编译期求值
   -  Constant可以被内联优化，Static不能被内联
   -  Static不允许包含任何析构，同时必须实现Sync,不能引用其他静态变量，Constant也不能引用其他静态变量
-  lazy_static包: 把定义全局静态变量延迟到运行时，而非编译时（可能这样就让Static不用在编译器求值，就能存储别的类型）
-  Metux与RwLock的区别？
   -  Metux：互斥锁，单线程读写
   -  RwLock：多线程读，单线程写，读写不能同时存在，只能一个线程占有锁，可以任意线程进行读

### 模块系统
- 为什么要进行模块化编程？
  - 易于维护
  - 隔离性：命名空间，缩小错误范围
  - 复用性
- 怎么声明一个模块？
  - 单个文件可以看作一个模块，在其他文件中引用这个模块使用`mod`关键字
  - 使用`mod`关键字声明一个模块，模块内作用域是私有的，需要使用`pub`关键字暴露
  - top-level模块中引用了一个模块，同级目录下的其他模块就不用再重新引用了
  - Rust2018, 文件夹与文件同名，同名文件夹下的文件为该文件的子模块
- Rust2015模块如何组织？
  - 类似文件系统的组织方式
  - 逻辑层面上尽量独立，不独立的合并为一个模块（放置在同一个文件夹中）
  - Cargo查找（根目录开始而不是相对路径，引用时需考虑这一点，添加`super::`）该文件夹下的mod.rs作为该模块的根文件
  - 在mod.rs中导入需要合并的模块
- Rust2018模块如何组织？
  - 添加子模块关系，文件夹与文件同名
  - 不再需要mod.rs
  - 使用crate关键字前缀，代表引用当前crate的模块，提高可读性

## 浏览《rust编程之道》第10章完整示例
- 浏览资料：
  - [StructOpt Doc](https://docs.rs/structopt/0.3.21/structopt/derive)
- 学习总结：
  - `#[derive(StructOpt)]`：让Rust生成命令行解析器
  - `#[structopt(..)]`: 附加的参数信息
    - `short`：短的选项名， 省略值则由字段名推断
    - `long`：长的选项名
    - `default_value`：该选项（对应struct字段）默认值
    - `help`：帮助信息

## 完成部分rustlings
- modules:
  - mod中定义的一切都是私有的，包括`use`