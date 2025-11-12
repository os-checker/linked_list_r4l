use linked_list_r4l::*;
use std::{sync::Arc, thread};

def_node! {
    struct Node(u8);
}

#[test]
fn pop_front_unsoundness() {
    // Adjust this number from small to large, you'll see more panics.
    const N: usize = 1 << 7;

    let data = Arc::new(Node::new(0));

    thread::scope(|s| {
        let mut v_thread = Vec::new();
        for id in 0..1 << 4 {
            let handle = thread::Builder::new()
                .name(id.to_string())
                .spawn_scoped(s, || {
                    let mut list = List::<Arc<Node>>::new();
                    for n in 0..N {
                        list.push_back(data.clone());
                        list.pop_front().unwrap_or_else(|| {
                            panic!("[{n}] This shouldn't happen, because we just push a node.")
                        });
                    }
                });
            v_thread.push(handle);
        }
        drop(v_thread);
    });
}
// thread '3' (2608321) panicked at tests/pop_front.rs:25:29:
// [0] This shouldn't happen, because we just push a node.
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// thread '8' (2608326) panicked at tests/pop_front.rs:25:29:
// [86] This shouldn't happen, because we just push a node.
//
// thread '11' (2608329) panicked at tests/pop_front.rs:25:29:
// [16] This shouldn't happen, because we just push a node.
//
// thread 'pop_front_unsoundness' (2608317) panicked at tests/pop_front.rs:15:5:
// a scoped thread panicked
// test pop_front_unsoundness ... FAILED
