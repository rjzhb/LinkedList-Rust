use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}
