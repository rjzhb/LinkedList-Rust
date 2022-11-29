include! {"LinkedList.rs"}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(2);
    list.push_front(3);

    list.print_out();
}
