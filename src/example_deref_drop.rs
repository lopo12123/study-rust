use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = (T);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("i`m going to be dropped!");
    }
}

fn say_hello(name: &str) {
    println!("hello {}", name);
}

#[cfg(test)]
fn test() {
    let x = 5;
    let my_box = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*my_box, 5);
    // ↑ 此处 *my_box 即为 *(my_box.deref()), 类似于运算符重载

    let my_name = MyBox::new(String::from("lopo"));

    // &my_name -> &MyBox<String> 类型
    // 自动执行 deref -> &String
    // 由于 String 类型实现了 Deref trait
    // 再次自动执行 deref -> &str
    say_hello(&my_name);
}