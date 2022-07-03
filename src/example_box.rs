use crate::example_box::List::{End, Node};

enum List {
    Node(i32, Box<List>),
    End,
}

#[cfg(test)]
fn test() {
    let demo_list = Node(1, Box::new(Node(2, Box::new(End))));
}