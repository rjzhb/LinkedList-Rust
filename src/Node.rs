
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: None,
            prev: None,
        })
    }

    fn next(&mut self, node: Box<Node>) {
        self.next = Some(node);
    }

    fn prev(&mut self, node: Box<Node>) {
        self.prev = Some(node);
    }
}
