# Day1 2021/7/2

## 学习《rust编程之道》


## 完成部分rustlings
### 问题
1. assert_eq(left: &String, right: &str)是怎么比较&str与String的？
   - 查阅资料:
     - [细谈Rust的所有权](https://zhuanlan.zhihu.com/p/115651233)
     - [Rust中的String和&str](https://zhuanlan.zhihu.com/p/123278299)
     - [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
     - [Rust Deref与自动解引用](https://www.jianshu.com/p/ad31c8f2e619)
   - 个人理解：
     - 栈存储的变量必须拥有确切的固定大小，而未知大小或大小可能改变的变量必须存储在堆中
     - 堆的工作原理：请求一个确定大小的空间，内存分配器找到一个足够大的空的空间，标记为已使用然后返回一个指针
     - 堆栈比较：因为需要寻找可用空间并记录，相对的栈分配内存则不需要，所以堆分配慢于栈；数据存储在堆上，访问它则需要由指针间接访问，同时需要寻找距离较远的指针数据，而栈内数据距离相对近，所以访问堆中数据比访问栈中数据要慢
     - rust使用所有权和借用来管理堆中数据
     - rust编译器发现不能通过编译时，会在三种情况下尝试自动解引用：
       - `&T => &U (T: Deref<Target=U>)`
       - `&mut T => &mut U (T:DerefMut<Target=U>`)
       - `&mut T => &U (T:Deref<Target=U>)`
     - `std::assert_eq` 使用PartialEq进行比较 其中`fn eq(&self, other: &Rhs) -> bool; &self: &String`自动解引用转换为`&str`类型，再进行比较

### 相关总结
1. variables:
   - Rust中，变量必须由`let`关键字声明
   - Rust变量必须先初始化再使用
   - 不能对immutable变量重新赋值
   - `as` 关键字只允许编译器认为合理的类型转换，如： `i8 => i32`，而不允许 `&str => i32`之类的转换
   - 对于类型不匹配的变量绑定，可以使用变量遮蔽绑定到一个新的变量上
   - `const`变量必须写明类型，不允许类型自动推导
2. functions:
   - Rust中，所有的函数形参都必须注明类型
   - 函数可以不写返回类型，但此时默认为unit `()`
   - 一个表达式加上;就变成了一条语句，语句的类型永远是unit`()`
3. if:
   - 语法：
     - `if condition {} else {}`
     - 这里一定要用大括号，避免”悬空else“ 
     - `if`和`else`的类型一定要一致
     - 若省略`else`分支则编译器默认认为`else`类型为`()`
4. move_semantics:
   - Rust中，变量间赋值默认为移动语义, 变量传递时，要么传递引用，要么复制出一个新的变量进行传递
5. primitive_types:
   - `bool`: `true` | `false`
   - `char`: `'single_char'`
   - `array`: `[type; size]` 初始化也是同样形式
   - `slice`: 配合`Range`使用，进行数组索引操作，`&array：Slice`： 胖指针， 携带长度信息，而不是像C语言一样的裸指针
   - 'tuple': `tuple`索引也是从0开始
   - 模式解构：构造与解构遵循相同的语法，怎么构造就怎么解构
6. structs:
   - `StructName; : unit()`
