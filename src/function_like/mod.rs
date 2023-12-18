use core::any::type_name;

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
    println!("size of foo is: {}", core::mem::size_of_val(&foo));  // size of foo is: 0
    println!("size of bar is: {}", core::mem::size_of_val(&bar));  // size of bar is: 0
    println!("type of foo is: \"{}\"", get_typename_of(&foo));  // type of foo is: "study_rust::function_like::foo"
    println!("type of bar is: \"{}\"", get_typename_of(&bar));  // type of bar is: "study_rust::function_like::bar"
}

#[test]
fn generic_test() {
    // usage
    assert_eq!(type_name::<i32>(), "i32");
    assert_eq!(type_name::<u32>(), "u32");
    assert_eq!(type_name::<String>(), "alloc::string::String");

    // zero size
    assert_eq!(core::mem::size_of_val(&type_name::<i32>), 0);
    assert_eq!(core::mem::size_of_val(&type_name::<u32>), 0);
    assert_eq!(core::mem::size_of_val(&type_name::<String>), 0);

    println!("all asserts passed");
}