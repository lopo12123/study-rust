enum List {
    Node(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Node, Nil};

#[cfg(test)]
fn test() {
    let a = Rc::new(Node(1,
                         Rc::new(Node(2,
                                      Rc::new(Nil))
                         ),
    ));
    println!("strong ref count: {}", Rc::strong_count(&a));  // 1

    let b = Node(3, Rc::clone(&a));
    println!("strong ref count: {}", Rc::strong_count(&a));  // 2

    {
        let c = Node(4, Rc::clone(&a));
        println!("strong ref count: {}", Rc::strong_count(&a));  // 3
    }

    println!("strong ref count: {}", Rc::strong_count(&a));  // 2
}