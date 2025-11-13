use linked_list_r4l::*;

def_node! {
    struct Node(u8);
}

#[test]
fn pop_front_unsoundness() {
    let mut list = List::<Box<Node>>::new();
    list.push_back(Box::new(Node::new(0)));
    list.pop_front().unwrap();
}
