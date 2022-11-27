include! {"Node.rs"}


#[derive(Debug, Clone)]
struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

#[allow(unused)]
impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() || self.tail.is_none()
    }
    

    fn push_front(&mut self, value: i32) {
        let mut new_head = Node::new(value);

        if let Some(mut node) = self.head.as_mut().take() {
            new_head.next(node.as_ref());
            node.prev(new_head);
            
        }
        self.head = Some(new_head);

    }

    fn push_back(&mut self, value: i32) {

    }
}
