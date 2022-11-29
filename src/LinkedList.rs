include! {"Node.rs"}

// use Self::Node::*;
// mod Node;

#[derive(Debug, Clone)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
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
        match &self.head {
            None => {
                let node = Rc::new(RefCell::new(Node::new(value)));
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
            Some(old_head) => {
                new_head.next = Some(Rc::clone(old_head));

                let node = Rc::new(RefCell::new(new_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&node));

                self.head = Some(node);
            }
        }
    }

    fn push_back(&mut self, value: i32) {
        let mut new_tail = Node::new(value);

        match &self.tail {
            None => {
                let node = Rc::new(RefCell::new(Node::new(value)));
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
            Some(old_tail) => {
                new_tail.prev = Some(Rc::downgrade(old_tail));

                let node = Rc::new(RefCell::new(new_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&node));

                self.tail = Some(node);
            }
        }
    }

    fn insert_next(&mut self, node: &mut Node){

    }

    fn insert_prev(&mut self, node: &mut Node){

    }

    fn print_out(&mut self) {
        if self.is_empty() {
            println!("List is empty");
        }

        let mut head = self.head.clone();
        println!("{:?}", head);

        loop {
            match head {
                None => break,
                Some(node) => {
                    println!("-> {:?}", node.borrow().value);
                    head = node.borrow().next.clone();
                }
            }
        }
    }
}
