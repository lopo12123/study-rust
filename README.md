## Cargo

> **新建项目** `cargo new project_name`

- 项目结构:

```yaml
project_name
- src
  - main.rs
- .gitignore
- Cargo.toml
- Cargo.lock
```

> **构建项目** `cargo build`

- 在目标路径生成可执行文件.
- 路径: `target/debug/project_name.exe` \(windows\)

**为发布构建** `cargo build --release`

- 编译时会进行优化, 代码会运行的更快, 但编译时间更长.
- 会在 `target/release` 下生成可执行文件, 而不是 `target/debug`

> **构建并运行项目** `cargo run`

- 如果项目编译过但未修改, 则会直接运行二进制文件, 不会重复编译.

> **检查代码** `cargo check`

- 检查代码, 确保能通过编译, 但不会产生任何可执行文件.
- `cargo check` 比 `cargo build` 快得多, 能连续反复的使用 `cargo check` 检查代码, 提高效率.

> 配置文件 `Cargo.toml` _Tom`s Obvious, Minimal Language_

- 常见字段含义:

```toml
### 包配置信息
[package]

# 项目名
name = "study-rust"

# 项目版本
version = "0.1.0"

# 使用的rust版本
edition = "2021"

# 软件包的许可证
license = "MIT"

# 作者
authors = ["lopo <lopo@zju.edu.cn>"]

# 对包的简要介绍说明
description = "a brief description of this package"

# 显式声明哪些文件被包含(排除)在内(外)
exclude = ["build/**/*.o", "doc/**/*.html"]
include = ["src/**/*", "Cargo.toml"]

# 更多信息 (url)
documentation = "..."
homepage = "..."
repository = "..."

# 指向README文件, 并会被保存在注册表数据库中
readme = "..."

### 依赖信息
[dependencies]

# 指定包的版本
hammer = "0.5.0"

# 指定包的版本范围
color = "> 0.6.0, < 0.8.0"
```

## Rust

### 变量

> **变量与可变性**

- 声明变量使用 `let` 关键字
- 默认情况下变量是不可变(_immutable_)的
- 声明变量时在变量前加上 `mut`, 使变量可变

```rust
fn mut_example() {
    let immut_variable = 1;
    let mut mut_variable = 1;
    // immut_variable = 2;  // panic
    mut_variable = 2;  // correct
}
```

> **变量与常量**

- 常量(_constant_)在绑定值后也是不可变的, 但它与不可变变量有很多区别:
    - 不可以使用 `mut`, 常量是永远不可变的
    - 声明常量使用 `const` 关键字, 他的类型必须被标注
    - 常量可以在任意作用域内进行声明, 包括全局作用域
    - 常量只可以绑定到常量表达式, 无法绑定到函数的调用结果或只能在运行时才能计算得到的值
- 在程序运行期间, 常量在其声明的作用域内一直有效
- 命名规范: **Rust**里的常量使用全大写字母, 单词间使用下划线分隔. (例: `const MAX_POINTS: u32 = 10_000;`)

```rust
fn const_example() {
    const MAX_POINTS: [i32; 2] = [0, 0];
    // const mut MAX_POINTS_MUT: [i32; 2] = [0, 0];  // error
}
```

> **重影 _Shadowing_**

- 可以使用相同的名字声明新的变量(新的类型、新的值), 新的变量会重影(`shadow`)之前声明的变量
- `shadow` 和把变量标记为 `mut`是不一样的:
    - 如果不使用 `let` 关键字, 那么重新给非 `mut` 的变量赋值会导致编译时错误
    - 而使用 `let` 声明同名新变量, 也是不可变的
    - 使用 `let` 声明的新变量, 可以为新的类型/新的值

```rust
fn shadow_example() {
    let what_type: u32 = 1;  // u32
    let what_type: &str = "1";  // &str
    let what_type: bool = false;  // bool
}
```

### 数据类型

- 标量和复合类型
- **Rust**是静态编译语言, 在编译时必须知道所有变量的类型
    - 基于使用的值, 编译器通常能够推断出它的具体类型
    - 但如果可能的类型比较多(如 `String` 的 `parse` 方法), 就必须添加类型标注, 否则就会报编译时错误

> **标量类型**

- 一个标量类型代表一个单个的值
- **Rust**有四个主要的标量类型
    - 整数类型
    - 浮点类型
    - 布尔类型
    - 字符类型

> > **整数类型**

- 整数类型没有小数部分
- 无符号以 `u` 开头
- 有符号以 `i` 开头
- **Rust**的整数类型列表如下

| length | signed | unsigned |
| --- | --- | --- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

- `izise` 和 `usize` 类型的位数由程序运行的计算机的架构决定
    - 如果是64位计算机, 那么就是64位的
    - 使用 `isize` 或 `usize` 的主要场景是对某种集合进行索引操作
- 整数字面量
    - 除了 `byte` 类型外, 所有的数值字面量都允许使用类型后缀 `42u8`
    - 如果不清楚该使用什么类型, 则可以使用相应的默认类型
        - 整数的默认类型是 `i32`

| 进制 | 例子 |
| --- | --- |
| Decimal | `10_000` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b11` |
| Byte(`u8` only) | `b'A'` |

- 整数溢出
    - 调试模式下编译, **Rust** 会检查整数溢出, 如果发生整数溢出则会导致 `panic`
    - 发布模式(`--release`)下编译, **Rust** 不会检查可能导致 `panic` 的整数溢出
        - 如果溢出发生, 则会执行 _"环绕"_ 操作 (例: `u8` 类型的 `256` 变成 `0`)

> > **浮点类型**

- 浮点类型含有小数部分
- **Rust** 有两种基础的浮点类型
    - `f32` 32位, 单精度
    - `f64` 64位, 双精度
- **Rust** 的浮点类型使用了 `IEEE-754` 标准来表述
- `f64` 是默认类型, 因为现代CPU上 `f64` 和 `f32` 速度差不多, 但精度更高
- 数值操作

```rust
fn calculate_example() {
    let sum = 5 + 10;
    let difference = 95.5 - 33.2;
    let product = 4 * 30;
    let quotient = 5.67 / 3.22;
    let remainder = 51 % 4;
}
```

> > **布尔类型**

- **Rust** 的布尔类型也有两个值: `true` 和 `false`
- 占用一个字节大小
- 符号为 `bool`

> > **字符类型**

- **Rust** 中的 `char` 类型被用来描述单个字符
- 字符类型的字面量使用单引号 `'`
- 占用4字节大小
- 是 `Unicode` 标量值, 可以表示比 `ASCII` 多得多的字符内容 (拼音、中日韩文、emoji表情等)
    - 范围为 `U+0000` 到 `U+D7FF` 和 `U+E000` 到 `U+10FFFF`
- 但 `Unicode` 中没有 _字符_ 的概念, 所以直觉上的字符与 **Rust** 中的字符概念并不相符

```rust
fn char_example() {
    let x = 'x';
    let y: char = 'お';
    let z = '😂';
}
```

> **复合类型**

- 复合类型可以将多个值放在一个类型里
- **Rust** 提供了两种基础的复合类型: 元组(`Tuple`) 和 数组(`Array`)

> > 元组

- 元组可以将多个类型的多个值放在一个类型中
- 元组的长度是固定的, 一旦声明就无法改变

```rust
fn tuple_example() {
    // 声明
    let tup: (i32, f64, u8) = (-500, 6.4, 1);

    // 获取值
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);  // -500, 6.4, 1

    // 访问
    println!("{}, {}, {}", tup.0, tup.1, tup.2);  // -500, 6.4, 1
}
```

> > 数组

- 数组也可以将多个类型的多个值放在一个类型中
- 数组的长度是固定的, 且其中的每个元素类型必须相同
- 数组的类型以这种形式表示: `[type; length]`
- \* 如果数组中的每一个值都相同, 那么:
    - 在中括号中指定初始值
    - 使用分号分隔
    - 最后写上数组的长度
    - `let a = [3; 5];` 等价于 `let a = [3, 3, 3, 3, 3];`
- 访问数组元素: 使用索引访问(索引从 `0` 开始)
- 如果访问的索引超出了数组的范围
    - 编译可能会通过
    - 运行会报错 (`panic`)
    - **Rust** 不允许其访问数组越界后相应地址的内存 (与 `c/c++` 区别)

```rust
fn array_example() {
    // 声明
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [1; 5];

    // 访问
    println!("a[1]: {}", a[1]);
    println!("b[2]: {}", b[2]);

    // 越界
    let names = ["name1", "name2", "name3"];
    let idx = [1, 2, 3, 4];
    println!("{}", names[4]);  // panic in compiling
    println!("{}", names[idx[3]]);  // runtime panic
}
```

### 函数

- 声明函数使用 `fn` 关键字
- 针对函数和变量名, **Rust** 使用 `snake case` 命名规范
    - 所有字母都小写, 单词间使用下划线分隔
- 函数体由一系列语句组成, 可选的由一个表达式结束
    - **语句** 是执行一系列动作的指令
    - **表达式** 会计算产生一个值
- 函数的返回值
    - 在 `->` 符号后面声明函数返回值的类型, 但是不可以为返回值命名
    - 在 **Rust** 中, 返回值就是函数体里的最后一个表达式的值
    - 若想提前返回, 则使用 `return` 关键字, 并指定一个值
        - 大多数函数都是默认使用最后一个表达式作为返回值

### 控制流

- `if` 表达式
    - `if` 表达式允许根据条件执行不同的分支代码, 这个条件**必须**是 `bool` 类型的
    - 三目用法 `let a = if condition { x } else { y };`
- `loop` 表达式
    - `loop` 关键字反复执行一块代码直到停止循环
    - 可以在 `loop` 循环中使用 `break` 关键字来告诉程序何时停止循环
    - `breal` 退出循环可以返回值 `let a = loop { break 3; };`
- `while` 条件循环
    - 每次循环前都判断一次条件
- `for` 循环

```rust
fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{}", element);
    }
}
```

### 所有权

- **Rust** 的核心特性就是所有权
- 所有语言在运行时都必须管理他们使用计算机内存的方式
    - 有的语言有垃圾收集机制(`Gabrage Collect, GC`), 在程序运行时会不断寻找不再使用的内存. 如: `C#`, `Java`, `JavaScript` 等
    - 有的语言必须显式地分配和释放内存. 如: `C`, `C++` 等
    - **Rust** 使用了第三种方式, 通过一个所有权系统来管理, 其包含了一组编译器在编译时检查的规则
- 栈`Stack` vs 堆`Heap`
    - `Stack` 按值的接受顺序来存储, 按相反顺序将他们移除(**LIFO**)
    - 所有存储在 `Stack` 上的数据必须拥有已知且固定的大小
        - 编译时大小未知的数据或运行时大小可能发生改变的数据都必须存放在 `Heap` 中
    - `Heap` 内存组织性差一些
        - 当把数据放入 `Heap` 时, 会请求一定数量的空间
        - 操作系统在 `Heap` 中找到一块足够大的空间, 将其标记为在用, 并返回该空间的地址
- 所有权解决的问题
    - 跟踪代码中哪些部分正在使用 `Heap` 的哪些数据
    - 最小化 `Heap` 上的重复数据量
    - 清理 `Heap` 上未使用的数据以避免空间不足
    - 所有权存在的原因: 管理 `Heap` 数据
- 所有权规则
    - 每个值都有一个变量, 这个变量是该值的**所有者**
    - 每个值同时只能有一个所有者
    - 当所有者超出**作用域**(`scope`)时, 该值将被删除
    - 变量的作用域就是程序中一个项目的有效范围
    - 当变量离开作用域时会自动执行一个 `drop` 函数

> 变量和数据交互的方式

- 移动(`Move`)
    - 多个变量可以与同一个数据使用一种独特的方式来交互
- 克隆(`Clone`)
- 复制(`Copy`)
    - `Copy trait` 可以用于像整数这样完全存放在 `Stack` 上的类型
    - 如果一个类型实现了 `Copy` 这个 `trait`, 那么复制之后旧的变量任然可用
    - 如果一个类型或该类型的一部分实现了 `Drop trait`, 那么 **Rust** 不允许让它再去实现 `Copy trait`
    - 任何简单标量的组合类型都是可以 `Copy` 的
    - 任何需要分配内存或某种资源的都不是可 `Copy` 的
    - 一些具有 `Copy trait` 的类型
        - 所有的整数类型, 如 `u32`
        - 所有的浮点类型, 如 `f64`
        - `bool`
        - `char`
        - `Tuple` (如果其所有字段都是可 `Copy` 的, 那么它也是可 `Copy` 的)

```rust
fn move_and_clone_example() {
    // 简单标量(在栈上的数据), 发生复制
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);  // x: 5, y: 5

    // 在堆上的数据, 所有权发生转移, s2赋值后s1将失效
    let s1 = String::new("rust");
    let s2 = s1;
    println!("s1: {}, s2: {}", s1, s2);  // panic: borrow of moved value "s1"

    // 克隆
    let s3 = s1.clone();
    println!("s1: {}, s3: {}", s1, s3);  // s1: rust, s3: rust
}
```

> 所有权与函数

- 在语义上, 将值传递给函数和将值赋值给变量是类似的(值会发生**移动**或**复制**)
- 函数的返回值在过程中同样也会发生所有权的转移
- 一个变量的所有权总是遵循相同的模式
    - 将一个值赋值给其他变量时就会发生移动
    - 当一个包含 `Heap` 的数据的变量离开作用域时, 它的值就会被 `drop` 函数清理(除非数据所有权移动到了另一个变量上)

> 引用和借用

- `&` 符号表示引用: 允许引用某些之但不获得其所有权
- 把引用作为函数参数的行为叫借用
- 不能修改借用的变量, 引用默认也是不可变的
- 可变引用 `&mut`
    - 可变引用有一个重要限制: 在特定作用域内, 对某一数据只能有一个可变引用(可以在编译时防止数据竞争)
    - 另一个限制: 不可以同时拥有一个可变引用和一(多)个不可变引用

> 切片

- **Rust** 的另一种不持有所有权的数据类型
- 形式: `&Origin[start, end]`
    - 开始索引就是切片起始位置的索引值
    - 结束索引是切片结束位置的下一个索引值
- 语法糖
    - 切片包含字符串头部: `&Origin[..end]`
    - 切片包含字符串尾部: `&Origin[start..]`
    - 切片包含字符串全部: `&Origin[..]`
- 字符串切片
    - 是指向字符串中一部分内容的引用
    - 字符串字面量的变量类型是 `&str`, 是一个指向二进制程序特定位置的切片, 是不可变引用
    - 注意
        - 字符串切片的范围索引必须发生在有效的 `UTF-8` 字符边界内
        - 如果尝试从一个多字节的字符中创建切片, 则会引发 `panic`

```rust
fn slice_example() {
    let string = String::from("Hello world!");
    let arr = [1, 2, 3, 4, 5];

    let str_slice = &string[1..4];  // ell
    let arr_slice = &arr[2..5];  // [3, 4, 5] 
}
```

### 结构体

- `struct` 结构体
    - 自定义的数据类型
    - 为相关联的值命名, 打包 => 有意义的组合
    - 一旦 `struct` 是可变的, 则结构体中的所有字段都是可变的
- 定义
    - 使用 `struct` 关键字, 并为整个结构体命名
    - 在花括号内为所有字段(`field`)定义名称和类型
- 实例化
    - 为每个字段指定具体值
    - 无需按声明的顺序指定
- 访问
    - 使用点标记法 `struct_name.field_name`
- 其他
    - 字段简写: 构造时同 `js`
    - 更新: 类似 `js` 对象解构
    - `Tuple struct`
        - 可以定义类似 `tuple` 的 `struct`, 叫做 `tuple struct`
        - `tuple struct` 整体有名, 但里面的元素没有名
        - 适用: 想给整个 `tuple` 起名, 并让它不同于其他 `tuple`, 而且又不需要给每个元素起名
    - `Unit-Like Struct` (没有任何字段)
        - 可以定义没有任何字段的 `struct`, 叫做 `Unit-Like struct`(因为与`()`, 单元类型类似)
        - 使用: 在某个类型上实现某个 `trait`, 但又没有数据需要存储

```rust
// 定义
struct User {
    username: String,
    email: String,
    account: u64,
    active: bool,
}

fn struct_example() {
    // 实例化
    let user = User {
        username: String::from("name"),
        email: String::from("email@example.com"),
        account: 123456789,
        active: true
    };
}

// tuple struct
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn tuple_struct_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black 和 origin 是不同类型(他们是不同的 tuple struct 的实例)
}
```

> `Struct` 的 方法 / 关联函数

- 方法和函数类似: `fn` 关键字、名称、参数、返回值
- 方法和函数不同之处
    - 方法是在 `struct`(或 `enum`, `trait` 对象)的上下文中定义 (在 `impl` 中实现)
    - 第一个参数是 `self`(或 `&self`, `&mut self`), 表示方法被调用的 `struct` 实例
- 可以在 `impl` 块里定义不把 `self` 作为第一个参数的函数, 他们叫做关联函数
    - 关联函数通常用于构造器(例: `String::from()`)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 枚举与模式匹配

> 枚举 `enum`

```rust
// 定义
enum IpKind {
    V4,
    V6,
}

enum IpKindValue {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 方法
impl IpKindValue {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// 使用
fn ip_enum_example() {
    let ipv4 = IpKind::V4;
    let ipv6 = IpKind::V6;

    let ipv4_val = IpKindValue::V4(0, 0, 0, 0);
    let ipv6_val = IpKindValue::V6(String::from("0::0"));

    ipv6_val.call();
}

// Option 枚举
// 处理其他语言的 null 情况
enum Option<T> {
    Some(T),
    None,
}
```

> 模式匹配

> > `match`

- 允许一个值与一系列模式进行匹配, 并执行匹配的模式对应的代码
- 模式可以是字面量、变量名、通配符等

```rust
fn match_example() {
    let v: u8 = 3;
    match v {
        // 一行语句: 可以直接写
        1 => println!("one"),
        // 多行语句: 包裹在块内
        2 => {
            println!("two");
            println!("two");
        }
        // 无关内容: 用 _ 通配匹配
        _ => {
            println!("more then 2");
        }
    }
}
```

> > `if let`

- 只关心一种匹配而忽略其他匹配的情况
- 更少的代码、更少的缩进、更少的模板代码
- 放弃了穷举的可能
- 可以搭配 `else`

```rust
fn if_let_example() {
    let v: u8 = 2;

    if let 0 = v % 2 {
        println!("even");
    } else {
        println!("odd");
    }
}
```

### 错误处理

`unwrap` 和 `expect`

- `unwrap(self)`:
    - 如果成功, 则直接返回 `Result::Ok` 里的值,
    - 如果失败, 则调用 `panic!` 宏中止程序.
- `expect(self, msg: &str)`:
    - 接受一个字符串参数(切片类型), 当结果为`Result::Error`的时候输出.
    - 如果成功, 则直接返回 `result` 里的值,
    - 如果失败, 则输出`expect`的入参并调用 `panic!` 宏中止程序.
- `result` 的结构如下:

```rust
enum Result<T, E> {
    Ok(T),
    Error(E),
}
```