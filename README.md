## Links

- [Rust 程序设计语言 简体中文版](https://rust.bootcss.com/title-page.html)
- [crates.io](https://crates.io/)

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

### 模块系统

- **Rust** 的模块系统
    - `Package`(包): `Cargo` 的特性, 能够构建、测试、共享 `crate`
    - `Crate`(单元包): 一个模块树, 可以产生一个 `library` 或可执行文件
    - `Module`(模块)、`use`: 能够控制代码的组织、作用域、私有路径
    - `Path`(路径): 为 `struct`、`function`、`module`等命名的方式

> `Cargo` 的惯例

- `src/main.rs` 是 `binary crate` 的 `crate root`, 且 `crate` 名与 `package` 名相同
- `src/lib.rs` 是 `library crate` 的 `crate root`, 且 `crate` 名与 `package` 名相同
- 一个 `package` 可以同时包含 `src/main.rs` 和 `src.lib.rs`
    - 一个 `binary crate`, 一个 `library crate`
- 一个 `package` 可以包含多个 `binary crate`
    - 文件放在 `src/bin`
    - 每个文件都是单独的 `binary crate`

> `Package`

- 一个 `Package` 包含 - **1** 个 `Cargo.toml`, 他描述了如何构建这些 `crate`
- 只能包含 **0-1** 个 `library crate`
- 可以包含 **任意数量** 的 `binary crate`
- 必须至少包含一个 `crate`(`binary` 或 `library`)

> `Crate`

- 类型: `binary` 或 `library`
- 作用: 将相关功能放到一个作用域内, 便于在项目间进行共享(防止冲突)
- `crate Root` 是源代码文件, **Rust** 编译器从这里开始构建 `crate` 根的 `Module`

> `Module`

- 在一个 `crate` 内, 将代码进行分组
- 使用 `mod` 关键字, 可嵌套
- 可以包含其他项(`struct`, `enum`, `trait`, 常量, 函数等)的定义

> `Path`

- 为了在 **Rust** 的模块中找到某个条目, 需要使用路径
- 路径的两种形式
    - 绝对路径: 从 `crate root` 开始, 使用 `crate` 名 或字面量 `crate`
    - 相对路径: 从当前模块开始, 使用 `self`, `super` 或当前模块的标识符
- 路径至少由一个标识符组成, 标识符之间使用 `::`

> 私有边界 `privacy boundary`

- 模块不仅可以组织代码, 还可以定义私有边界
- 如果想把函数 或 `struct` 等设为私有, 可以将它放到某个模块中
- **Rust** 中的所有条目(函数, 方法, 常量, 模块, `struct`, `enum` 等)默认是私有的
- 父级模块无法访问子模块中的私有条目
- 子模块可以使用所有祖先模块中的条目
- 模块定义时, 如果模块后面是 `;` 而不是代码块
    - **Rust** 会从与模块同名的文件中加载内容
    - 模块树的结构不会变化
- 关键字
    - `pub` 暴露公共
    - `super` 访问父级模块中的内容, 类似文件系统中的 `..`
    - `pub struct`
        - `struct` 是公共的
        - `struct` 的字段默认是私有的
    - `pub enum`
        - `enum` 是公共的
        - `enum` 的变体默认也是公共的
    - `use`
        - 可以使用 `use` 关键字将路径导入到作用域内
        - 仍遵循私有性原则
        - `as`: alias, like js (esm: `import * as customName from "..."`)
        - `pub use`: re-export, like js (esm: `export * from "..."`)
        - `use std::{self, fmt, cmp};`: part-use, like js (esm: `import { classA, classB } from "...""`)
        - `use std::*;`: use all, like js (esm: `import * from "..."`)

```rust
/// module_in_other_file.file
pub fn function_in_other_file() {}

/// lib.rs
// 用下行语句引入外部文件的 module
// mod module_in_other_file;

mod outer_module {
    pub mod inner_module {
        pub fn inner_function() {}
    }

    pub struct StructExample {
        private_item: i32,
        pub public_item: i32,
    }

    pub enum EnumExample {
        DefaultToPublic,
        NoNeedOfPub
    }

    fn super_example() {
        super::path_example()
    }
}

pub fn path_example() {
    // absolute
    crate::outer_module::inner_module::inner_function();

    // relative
    outer_module::inner_module::inner_function();
}

fn use_example() {
    use outer_module::inner_module;

    inner_module::inner_function();
}
```

### 常用集合

> `Vector`

- 类型为 `Vec<T>`
- 由标准库提供
- 可存储多个值
- 只能存储相同类型的数据
- 值在内存中连续存放

```rust
fn vector_example() {
    // 创建
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    // 更新
    v.push(4);
    v.push(5);

    // 引用
    // - 索引访问 (越界会 panic)
    let third_1: &i32 = &v[2];
    // - get 访问 (越界则为 None)
    let third_2 = match v.get(2) {
        Some(val) => val,
        None => None
    };

    // 遍历 - for 循环
    // - 不可变引用 循环
    for val in &v {
        print!("{}", val);  // -> 1 2 3 4 5
    }
    // - 可变引用 循环
    for val in &mut v {
        *val += 10;
        print!("{}", i);  // -> 11 12 13 14 15
    }
}
```

> `String`

- **Rust** 的 **核心语言层面**, 只有一个字符串类型: 字符串切片 `&str`
    - 字符串切片: 对存储在其他地方的 `UTF-8` 编码的字符串的引用
    - 字符串字面量: 存储在二进制文件中的字符串切片
- `String` 类型
    - 来自 **标准库** 而不是核心语言
    - 可增长、可修改、可获得所有权
    - `UTF-8` 编码
    - 是对 `Vec<u8>` 的包装, `len()` 方法 返回的是 `Unicode` 标量值, 而非常规意义的字符数 (不支持索引形式访问, 用索引方式进行访问会报错)
- 字节`Bytes`、标量值`Scalar Values`、字形簇`Grapheme Clusters`
    - 遍历字节 `for b in s.bytes() {}`
    - 遍历`Unicode`标量值 `for c in s.chars() {}`
    - 遍历字形簇 `无标准库提供`

```rust
fn string_example() {
    // 创建
    // - 使用 ::new 方法创建
    let s1 = String::new();
    // - 使用 to_string 方法
    let s2 = "hello".to_string();
    // - 使用 ::from 方法创建
    let mut s3 = String::from("hello");

    // 更新
    // - push_str 把字符串切片附加到 String
    s3.push_str(" world");
    println!("{}", s3);  // -> hello world
    // - push 把单个字符附加到 String
    s3.push('!');
    println!("{}", s3);  // -> hello world!
    // - '+' 运算符
    // 使用了类似 fn add(self, s: &str) -> String {...} 的方法, 第一个字符串失去其所有权
    let name = String::from(" bye.");
    let s4 = s3 + &name;  // -> hello world! bye. (s3 发生了移动已失效)
    // - format! 宏 (类似 js 字符串模板语法)
    let s5 = format!("{}-{}", s2, s4);
    println!("{}", s5);  // -> hello-hello world! bye

    // 访问
    // - 遍历字节 
    for b in s5.bytes() {}
    // - 遍历`Unicode`标量值 
    for c in s5.chars() {}
    // - 遍历字形簇 无标准库提供

    // 切割
    // 允许 但不能跨越字符边界, 如果切割了字符边界则会引发 panic
    // - &string[start..end]
    let string_part = &s5[0..3];
}
```

> `HashMap`

- 类型 `HashMap<K, V>`
- 键值对的形式存储, 一个键 `K` 对应一个值 `V`
- 适用场景: 通过 `K` 来寻找数据, 而不是通过索引
- 所有权
    - 对实现了 `Copy trait` 的类型(如`i32`), 值会被复制到 `HashMap` 中
    - 对拥有所有权的值, 其值会被移动, 所有权转移给 `HashMap`
        - 如果将值的引用插入到 `HashMap` 中, 值本身不会移动
        - 但在 `HashMap` 有效期间, 被引用的值必须保持有效
- 默认情况下, `HashMap` 使用加密功能强大的 `Hash` 函数, 可以抵抗 `Dos` 攻击
    - 不是可用的最快的 `Hash` 算法, 但具有更好的安全性
    - 可以指定不同的 `hasher` 来切换到另一个函数, `hasher` 是实现 `BuildHasher trait` 的类型

```rust
use std::collections::HashMap;

fn hashmap_example() {
    // 创建
    // - 使用 ::new 方法创建
    let mut map: HashMap<&String, i32> = HashMap::new();
    // - 使用 ::collect 方法创建
    // 在元素类型为 Tuple 的 Vector 上使用 collect 方法, 可以组建一个 HashMap
    let teams = vec![String::from("red"), String::from("blue")];
    let scores = vec![10, 50];
    let team_with_score: HashMap<_, _> = teams.iter()
        .zip(scores.iter())
        .collect();

    // 添加数据
    let red_team = String::from("red");
    map.insert(&red_team, 10);

    // 访问数据
    let red_score = match map.get(&red_team) {
        Some(score) => score,
        None => None,
    };

    // 遍历 - for 循环
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    // 更新
    // - K 已经存在, 对应一个 V
    // - - 替换现有的 V
    map.insert(&red_team, 20);
    // - - 保留现有的 V, 忽略新的 V
    map.entry(&red_team).or_insert(30);  // 已存在, 忽略
    let blue_team = String::from("blue");
    map.entry(&blue_team).or_insert(40);  // 不存在, 插入
    // - - 合并现有的 V 和新的 V
    let text = "a b c a c";
    let mut char_map = HashMap::new();
    for char in text.split_whitespace() {
        let count = char_map.entry(char).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", char_map);  // {"a": 2, "b": 1, "c": 2}
    // - K 不存在: 增加一个 K-V 对
    let new_team = String::from("new");
    map.insert(&new_team, 100);
}
```

### 错误处理

> **可恢复错误** 和 **不可恢复错误**

- 可恢复错误 `Result<T, E>`
    - 例: 文件未找到, 可再次尝试
    - `Result` 枚举
- 不可恢复错误 `panic!`
    - 例: 访问的索引越界
    - 执行 `panic!` 宏
        - 打印一个错误信息
        - 展开 `unwind`, 清理调用栈 `stack`
        - 退出程序

```rust
// Result 结构如下
// enum Result<T, E> {
//     // 成功返回值
//     Ok(T),
//     // 失败返回值
//     Err(E),
// }
fn result_example() {
    let f = File::open("hello.txt");

    // 匹配文件打开结果
    let f = match f {
        Ok(file) => file,
        // 匹配错误类型
        Err(err) => match err.kind() {
            ErrorKind::NotFound => println!("file not found!"),
            other_error => panic!("other error!"),
        }
    };

    // 使用闭包重写上述代码
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() - ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!( "Error creating file: {:?}"， error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });
}
```

> `unwrap` 和 `expect` 和 `?`运算符

- `unwrap(self)`:
    - 如果成功, 则直接返回 `Result::Ok` 里的值,
    - 如果失败, 则调用 `panic!` 宏中止程序.
- `expect(self, msg: &str)`:
    - 接受一个字符串参数(切片类型), 当结果为`Result::Error`的时候输出.
    - 如果成功, 则直接返回 `result` 里的值,
    - 如果失败, 则输出`expect`的入参并调用 `panic!` 宏中止程序.
- `?`运算符
    - 在返回类型为 `Result` 的函数中使用
    - 在 `Result` 类型的值后面使用
    - 可链式调用, 类似于 `typescript` 中的 `?.` 操作
    - 如果成功, `Ok(res)` 中的 `res`作为表达式的结果, 函数继续执行
    - 如果失败, 整个函数返回 `Err(err)`, 相当于执行了 `return Err(err)`

### 泛型、`Trait`、生命周期

> 泛型

- 提高代码复用能力
- 是具体类型或其他属性的抽象代替
- 与 `typescript` 类似
    - `fn` -> `function`
    - `struct` -> `class`
    - `enum` -> `type` / `interface`
- `fn largest<T>(list: &[T]) -> T {...}`
- 性能和普通代码一致, 因为编译器会执行单态化(类似`c++`)

> `Trait`

- 把方法签名放在一起, 定义实现某种目的所必须的一组行为 (类似抽象类)
    - 关键字 `trait`
    - 只有方法签名, 没有具体实现
    - `trait` 可以有多个方法: 每个方法签名占一行, 以 `;` 结尾
    - 实现该 `trait` 的类型必须提供具体的方法实现
- 实现 `trait` 的约束
    - 这个类型或这个 `trait` 是在本地 `crate` 里定义的
    - 无法为外部类型实现外部的 `trait` (孤儿规则)
- 默认实现
    - 在 `trait` 中直接实现方法, 作为默认实现
    - 默认实现中的方法可以调用 `trait` 中其他的方法, 即使这些方法没用默认实现
- `trait` 作为参数
    - `impl trait` 语法, 适用于简单情况 (`trait bound` 的一种语法糖)
    - `trait bound` 语法, 适用于复杂情况
    - 使用 `+` 指定多个 `trait bound`
    - `trait bound` 可使用 `where` 子句
- `trait` 作为返回类型
    - `impl trait` 语法: 只能返回确定的同一种类型, 返回可能不同类型的代码会报错

```rust
pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
}

pub trait Display {
    fn display1(&self) -> String;
    fn display2(&self) -> String;
}

// trait 作为参数
fn impl_trait_example(item: impl Summary + Display) {
    println!("{}", item.summarize1());
}

fn trait_bound_example<T: Summary + Display>(item: T) {
    println!("{}", item.summarize1());
}

fn where_example<T, K>(item1: T, item2: K)
    where
        T: Summary,
        K: Summary + Display,
{
    println!("{}, {}", item1.summarize1(), item2.display1());
}

// trait 作为返回类型
fn trait_as_result_example() -> impl Summary {
    // native code
}
```

> 生命周期

- 引用保持有效的作用域
    - 避免垂悬引用 (`dangling reference`)
    - 每个引用都有自己的生命周期
    - 大多数情况是隐式的、可推断的
    - 当引用的生命周期可能以不同的方式互相关联时, 可以手动标注生命周期
- 借用检查器
    - 比较作用域来判断所有的借用是否合法
- 生命周期标注
    - 生命周期的标注不会改变引用的生命周期长度
    - 当指定了泛型生命周期参数, 函数可以接收带有任何生命周期的引用
    - 生命周期的标注描述了多个引用的生命周期之间的关系, 但不影响生命周期
    - 语法
        - 参数名以 `'` 开头, 通常全小写非常短(`'a`)
        - `&i32`  // 一个引用
        - `&'a i32`  // 带有显式生命周期的引用
        - `&'a mut i32`  // 带有显式生命周期的可变引用
- 生命周期省略的三个规则 (适用于 `fn` 和 `impl`)
    - 规则1: 每个引用类型的参数都有自己的生命周期
    - 规则2: 如果只有一个输入生命周期参数, 那么该生命周期被赋给所有的输出生命周期参数
    - 规则3: (只适用于方法) 如果有多个输入生命周期参数, 但其中一个是 `&self` 或 `&mut self`, 那么 `self` 的生命周期会被赋给所有的输出生命周期参数
- 函数定义中的声明周期标注
    - 泛型生命周期参数声明在在函数名和参数列表之间的 `<>` 里
    - 指定生命周期参数的方式依赖于函数所做的事
        - 从函数返回引用时, 返回类型的生命周期参数需要与其中一个参数的生命周期匹配
        - 如果返回的引用没有指向任何参数, 那么它只能引用函数内创建的值(发生了悬垂引用, 因为该值在函数结束时结束了其生命周期)
- 结构体定义中的生命周期标注(见下 `code example`)
- 方法定义中的生命周期标注
    - 在 `struct` 上使用生命周期实现方法, 语法和泛型参数的语法一致
    - `struct` 字段的生命周期名
        - 在 `impl` 后声明
        - 在 `struct` 后使用
        - 这些生命周期参数是 `struct` 类型的一部分
    - `impl` 块内的方法签名中
        - 引用必须绑定于 `struct` 字段引用的生命周期, 或者引用是独立的也可以
        - 生命周期省略规则经常使得方法中的生命周期标注不是必须的
- 静态生命周期
    - `'static` 是一个特殊的生命周期, 是整个程序的持续时间
        - 例如: 所有的字符串字面量都拥有 `'static` 的生命周期

```rust
// 生命周期错误示例
fn life_circle_error_example() {
    let r;
    {
        let x = 5;
        r = &x;  // error[E0597]: borrowed value does not live long enough
    }
    println!("r: {}", r);
}

// 生命周期标注实例
fn life_circle_mark_example() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);

    println!("longest string is: {}", result);
}

// 此处的生命周期 'a 的实际生命周期是: x 和 y 两个生命周期中较小的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 生命周期参数省略规则

// 例1
// 原始
fn ex1(s: &str) -> &str { "native code" }

// 规则1
fn ex1_mid<'a>(s: &'a str) -> &str { "native code" }

// 规则2
fn ex1_auto<'a>(s: &'a str) -> &'a str { "native code" }
// 此时符合 [所有引用都有其生命周期] -> 即可以省略

// 例2
// 原始
fn ex2(x: &str, y: &str) -> &str { "native code" }

// 规则1, 规则23不适用
fn ex2_auto<'a, 'b>(x: &'a str, y: &'b str) -> &str { "native code" }
// 此时不符合 [所有引用都有其生命周期] -> 即不可以省略

// 例3
struct LifeCircleMark<'a> {
    part: &'a str
}

impl<'a> LifeCircleMark<'a> {
    // 无引用返回, 则无需生命周期标注
    fn no_ref_return(&self) -> i32 { 1 }

    // 根据规则3, 自动为返回类型标注为 self 的生命周期
    fn with_ref_return(&self, arg1: &str) -> &str {
        "native code"
    }
}
```

```rust
// 综合使用
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>
(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() { x } else { y }
}
```

### 测试

- 使用 `#[test]` 标注所有的测试函数
- 使用 `cargo test` 命令运行所有的测试函数
    - 构建一个 `Test Runner` 可执行文件
    - 运行标注了 `test` 的函数并报告其是否成功
- 当使用 `cargo` 创建 `library` 项目时, 会生成一个 `test module`, 里面有一个 `test` 函数
    - 可以有任意数量的 `test module` 和 `test` 函数

> 断言

- `assert!` 测试 `bool`
    - 断言失败: 输出 `FAILED`
- `assert_eq!` 和 `assert_ne!` 测试相等性
    - 两个参数
    - 要求参数实现了 `PartialEq` 和 `Debug` 这两个 `trait` (所有的基本类型和标准库的大部分类型都已实现)
    - 断言失败: 输出 两个参数的值
- 添加自定义错误信息
    - 在 `assert!` 的第二个参数和 `assert_eq!`/`assert_ne!` 的第三个参数添加自定义信息 (可以使用占位符添加更多信息)
- 使用 `#[should_panic]` 标注应该发生 `panic`
    - `#[should_panic(expected = "expected error msg")]` 标注发生 `panic` 的错误信息应该包含某些内容文本
- 使用 `Result<T, E>` (无需 `panic`)
    - 返回 `Ok`: 测试通过
    - 返回 `Err`: 测试失败

> `cargo test`

- 默认行为
    - 并行运行
    - 执行所有测试
    - 捕获(不显示)所有输出, 使读取与测试结果相关的输出更容易
- 命令行参数
    - 针对 `cargo`: 放在 `cargo test` 之后
        - 使用 `cargo test --help` 查看可用信息
    - 针对测试可执行程序: 放在 `--` 之后
        - 使用 `cargo test -- --help` 查看可用信息
- 并行/串行 运行测试
    - 并行(默认)
        - 运行更快
        - 需要确保测试之间不互相依赖且不依赖于某一共享状态
    - 串行(控制线程数量)
        - `cargo test -- --test-threads=1`
- 显式函数输出
    - 如果测试通过, 默认会捕获所有标准输出内容(不显示)
    - 使用 `cargo test -- --show-output` 在成功的测试中显示输出
- 按名称运行测试
    - 将测试的名称(一个或多个)作为 `cargo test` 的参数
    - `cargo test name_of_test_fn`
    - 用测试名的一部分匹配多个测试
        - 如: 使用 `cargo test my_test` 匹配 `my_test_1`、`my_test_2` 等多个测试函数
    - 忽略某些测试, 运行剩余测试
        - 使用 `#[ignore]` 标记忽略测试函数
        - 使用 `cargo test -- --ignored` 只运行被标记为忽略的测试函数

> 单元测试 / 集成测试

- 单元测试
    - 一次对一个模块进行隔离的测试
    - 可测试 `private` 接口
    - 一般与模块同文件下, 使用 `#[cfg(test)]` 进行标注
        - 只有运行 `cargo test` 才会编译运行代码
        - 运行 `cargo build` 不会编译/运行 代码
    - 测试私有函数
        - 允许直接调用私有函数
- 集成测试
    - 和外部代码一样调用模块(可能使用到多个模块)
    - 只能测试 `public` 接口
    - `tests` 目录(与 `src` 并列)
        - 不需要 `#[cfg(test)]` 标注
        - 只会在使用 `cargo test` 时编译
        - 每一个文件都是一个单独的 `crate`
    - 运行指定的集成测试
        - 运行特定的集成测试: `cargo test name_of_test_fn`
        - 运行某个测试文件内的所有测试: `cargo test --test filename`
- 针对 `binary crate` 的集成测试
    - 如果项目是 `binary crate`, 只含有 `src/main.rs` 而没有 `src/lib.rs`
        - 不能在 `tests` 目录下创建集成测试
        - 无法把 `main.rs` 的函数导入到作用域
            - 只有 `library crate` 才能暴露函数给其他 `crate` 使用
            - `binary crate` 意味着独立运行
- `#[cfg(condition)]`: `configuration`(配置)
    - 只有在指定的配置条件下才被包含