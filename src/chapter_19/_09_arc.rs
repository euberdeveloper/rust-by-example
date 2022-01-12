use std::sync::Arc;
use std::thread;

pub fn run() {
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    let mut children = vec![];

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);

        children.push(thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
