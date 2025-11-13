use linked_list_r4l::*;
use std::{sync::Arc, thread};

def_node! {
    struct Node(String);
}

#[test]
fn mutable_arc() {
    let mut list = List::<Arc<Node>>::new();
    let data = Arc::new(Node::new("Hello".to_owned()));
    list.push_back(data.clone());

    // Modify data behind Arc without synchronization!
    // cc https://github.com/Rust-for-Linux/linux/issues/944
    //
    // "Shared references in Rust disallow mutation by default, and Arc is no exception:
    // you cannot generally obtain a mutable reference to something inside an Arc. If you
    // do need to mutate through an Arc, you have several options: ..."
    // src: https://doc.rust-lang.org/std/sync/struct.Arc.html
    let mut cursor = list.cursor_front_mut();
    let val = cursor.current().unwrap();
    val.inner = "world".to_owned();

    dbg!(list.pop_front().unwrap().inner());
    dbg!(data.inner());
}

#[test]
fn arc_data_race() {
    let data = Arc::new(String::new());

    let task = |id: usize| {
        let data = data.clone();
        const N: usize = 100;
        let f = move || {
            let ptr = Arc::into_raw(data) as *mut _;
            for i in N * id..N * (id + 1) {
                unsafe { *ptr = i.to_string() };
            }
        };
        thread::Builder::new()
            .name(id.to_string())
            .spawn(f)
            .unwrap()
    };

    let mut tasks = Vec::new();
    for id in 0..10 {
        tasks.push(task(id));
        println!("id={id} buf={data}");
    }

    for t in tasks {
        t.join().unwrap();
    }
}

#[test]
fn cursor_mut_unsoundness() {
    let data = Arc::new(Node::new(String::new()));

    let task = |id: usize| {
        let data = data.clone();
        const N: usize = 100;
        let f = move || {
            let mut list = List::<Arc<Node>>::new();
            list.push_back(data);
            let mut cursor = list.cursor_front_mut();
            let buf = &mut cursor.current().expect("No current corsor").inner;
            for i in N * id..N * (id + 1) {
                *buf = i.to_string();
            }
        };
        thread::Builder::new()
            .name(id.to_string())
            .spawn(f)
            .unwrap()
    };

    let mut tasks = Vec::new();
    for id in 0..10 {
        tasks.push(task(id));
        println!("id={id} buf={}", data.inner());
    }

    for t in tasks {
        t.join().unwrap();
    }
}
// id=5 buf=399
// id=6 buf=�
// id=7 buf=599
// id=8 buf=699
// id=9 buf=799
//
// thread '9' (2605626) panicked at tests/cursor_mut.rs:40:45:
// No current corsor
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// thread 'cursor_mut_unsoundness' (2605616) panicked at tests/cursor_mut.rs:58:18:
// called `Result::unwrap()` on an `Err` value: Any { .. }
// test cursor_mut_unsoundness ... FAILED
