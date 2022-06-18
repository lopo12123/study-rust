### Cargo

> `cargo new`

**新建项目** `cargo new project_name`

- 项目结构:

```yaml
project_name
- src
  - main.rs
- .gitignore
- Cargo.toml
- Cargo.lock
```

> `cargo build`

**构建项目** `cargo build`

- 在目标路径生成可执行文件.
- 路径: `target/debug/project_name.exe` \(windows\)

**为发布构建** `cargo build --release`

- 编译时会进行优化, 代码会运行的更快, 但编译时间更长.
- 会在 `target/release` 下生成可执行文件, 而不是 `target/debug`

> `cargo run`

**构建并运行项目** `cargo run`

- 如果项目编译过但未修改, 则会直接运行二进制文件, 不会重复编译.

> `cargo check`

**检查代码** `cargo check`

- 检查代码, 确保能通过编译, 但不会产生任何可执行文件.
- `cargo check` 比 `cargo build` 快得多, 能连续反复的使用 `cargo check` 检查代码, 提高效率.

> `Cargo.toml` _Tom`s Obvious, Minimal Language_

Cargo项目的配置文件格式

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

### Rust

> 变量

**变量与可变性**

- 声明变量使用 `let` 关键字
- 默认情况下变量是不可变(_immutable_)的
- 声明变量时在变量前加上 `mut`, 使变量可变

```rust
fn mut_and_immut() {
    let immut_variable = 1;
    let mut mut_variable = 1;
    // immut_variable = 2;  // panic
    mut_variable = 2;  // correct
}
```

**变量与常量**

- 常量(_constant_)在绑定值后也是不可变的, 但它与不可变变量有很多区别:
    - 不可以使用 `mut`, 常量是永远不可变的
    - 声明常量使用 `const` 关键字, 他的类型必须被标注
    - 常量可以在任意作用域内进行声明, 包括全局作用域
    - 常量只可以绑定到常量表达式, 无法绑定到函数的调用结果或只能在运行时才能计算得到的值
- 在程序运行期间, 常量在其声明的作用域内一直有效
- 命名规范: **Rust**里的常量使用全大写字母, 单词间使用下划线分隔. 例: `const MAX_POINTS: u32 = 10_000;`

```rust
fn var_and_const() {
    const MAX_POINTS: [i32; 2] = [0, 0];
    // const mut MAX_POINTS_MUT: [i32; 2] = [0, 0];  // error
}
```

**重影 _Shadowing_**

- 可以使用相同的名字声明新的变量(新的类型、新的值), 新的变量会重影(`shadow`)之前声明的变量
- `shadow` 和把变量标记为 `mut`是不一样的:
    - 如果不使用 `let` 关键字, 那么重新给非 `mut` 的变量赋值会导致编译时错误
    - 而使用 `let` 声明同名新变量, 也是不可变的
    - 使用 `let` 声明的新变量, 可以为新的类型/新的值

```rust
fn shadow() {
    let what_type: u32 = 1;  // u32
    let what_type: &str = "1";  // &str
    let what_type: bool = false;  // bool
}
```

> 错误处理

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