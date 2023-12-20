use core::{
    any::type_name,
    mem::{size_of, size_of_val},
};

fn get_typename_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn foo() -> i32 {
    42
}

fn bar() -> i32 {
    35
}

#[test]
fn function_item_example() {
    // 1. 零大小
    println!("size of foo is: {}", size_of_val(&foo));
    // => size of foo is: 0

    println!("size of bar is: {}", size_of_val(&bar));
    // => size of bar is: 0

    println!("size of function with generic type 1: {}", size_of_val(&get_typename_of::<i32>));
    // => size of function with generic type 1: 0

    println!("size of function with generic type 2: {}", size_of_val(&get_typename_of::<u32>));
    // => size of function with generic type 2: 0

    // 2. 类型
    println!("type of foo is: \"{}\"", get_typename_of(&foo));
    // => type of foo is: "study_rust::function_like::foo"

    println!("type of bar is: \"{}\"", get_typename_of(&bar));
    // => type of bar is: "study_rust::function_like::bar"
}

// ========== ========== ========== ==========

type MyFunctionPointer = fn();

fn hi() {
    println!("hi");
}

fn hello() {
    println!("hello");
}

#[test]
fn function_pointer_test() {
    // 1. coercion from function item
    let mut greet: MyFunctionPointer = hi;
    greet();
    // => hi
    println!("size of greet is: {}", size_of_val(&greet));
    // => size of greet is: 8

    // 2. re-assign is ok
    greet = hello;
    greet();
    // => hello
    println!("size of greet is: {}", size_of_val(&greet));
    // => size of greet is: 8

    // 3. coercion from non-capturing closure
    greet = || println!("nice day, isn't it?");
    greet();
    // => nice day, isn't it?
    println!("size of greet is: {}", size_of_val(&greet));
    // => size of greet is: 8

    // 4. coercion in pattern matching
    let condition = true;
    let my_size_of = if condition {
        size_of::<i32>
    } else {
        size_of::<u32>
    };
    println!("size of size_of::<i32> is: {}", size_of_val(&size_of::<i32>));
    println!("size of size_of::<u32> is: {}", size_of_val(&size_of::<u32>));
    println!("size of my_size_of is: {}", size_of_val(&my_size_of));
    // => size of size_of::<i32> is: 0
    // => size of size_of::<u32> is: 0
    // => size of my_size_of is: 8

    // EXTRA: size of usize
    println!("size of 'usize' is: {}", size_of::<usize>());
    // => size of 'usize' is: 8
}

// ========== ========== ========== ==========

/// alternative to non-capturing closure
struct Alternative1 {}

/// alternative to capturing closure -- 1 argument
#[allow(unused)]
struct Alternative2 {
    arg1: String,
}

/// alternative to capturing closure -- 2 argument (1)
#[allow(unused)]
struct Alternative3 {
    arg1: String,
    arg2: u32,
}

/// alternative to capturing closure -- 2 argument (2)
#[allow(unused)]
struct Alternative4<'a> {
    arg1: &'a String,
    arg2: u32,
}

#[test]
fn closure_test() {
    // 1. non-capturing closure
    let closure1 = || println!("no args");
    let alt1 = Alternative1 {};
    println!("closure: {}; struct: {}; (): {}", size_of_val(&closure1), size_of_val(&alt1), size_of::<()>());
    // => closure: 0; struct: 0; (): 0

    // 2. capturing closure (1)
    let arg1 = "abc".to_string();
    let closure2 = || {
        // consume arg1 to ensure it's captured
        let _args = arg1;
        println!("args: {:?}", _args);
    };
    let alt2 = Alternative2 { arg1: "abc".to_string() };
    println!("closure: {}; struct: {}; String: {}", size_of_val(&closure2), size_of_val(&alt2), size_of::<String>());
    // => closure: 24; struct: 24; String: 24

    // 3. capturing closure (2)
    let arg1 = "abc".to_string();
    let arg2 = 123_u32;
    let closure3 = || {
        let _args = (arg1, arg2);
        println!("args: {:?}", _args);
    };
    let alt3 = Alternative3 { arg1: "abc".to_string(), arg2: 123_u32 };
    println!("closure: {}; struct: {}; String: {}; u32: {}", size_of_val(&closure3), size_of_val(&alt3), size_of::<String>(), size_of::<u32>());
    // => closure: 32; struct: 32; String: 24; u32: 4

    // 4. capturing closure (3)
    let arg1 = "abc".to_string();
    let arg2 = 123_u32;
    let closure4 = || {
        let _args = (&arg1, arg2);
        println!("args: {:?}", _args);
    };
    let alt4 = Alternative4 { arg1: &arg1, arg2: 123_u32 };
    println!("closure: {}; struct: {}; &String: {}; u32: {}", size_of_val(&closure4), size_of_val(&alt4), size_of::<&String>(), size_of::<u32>());
    // closure: 16; struct: 16; &String: 8; u32: 4
}
// ========== ========== ========== ==========

#[test]
fn size_compare() {
    let f1 = hello;
    let f2: fn() = hello;
    let f3 = || println!("nice day, isn't it?");

    println!("size of f1 is: {}", size_of_val(&f1));
    println!("size of f2 is: {}", size_of_val(&f2));
    println!("size of f3 is: {}", size_of_val(&f3));
}