pub fn run() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        Nil
    }

    fn prepend(self, el: u32) -> Self {
        Cons(el, Box::new(self))
    }

    fn len(&self) -> u64 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }

        // match *self {
        //     Cons(_, ref tail) => 1 + tail.len(),
        //     Nil => 0
        // }
        // ^ This is the same
    }

    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => format!("{head} {tail}", head = head, tail = tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}
