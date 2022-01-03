use std::fmt::{self, Display, Formatter};

pub fn run() {
    let list = List(vec![1, 2, 3]);
    println!("The list is {}", list);

    let list2 = List2(vec![1, 2, 3]);
    println!("The list is {}", list2);
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;

        let list = &self.0;
        for (i, el) in list.iter().enumerate() {
            match i {
                0 => write!(f, "{}", el)?,
                _ => write!(f, ", {}", el)?,
            };
        }

        write!(f, " ]")
    }
}

struct List2(Vec<i32>);

impl Display for List2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;

        let list = &self.0;
        for (i, el) in list.iter().enumerate() {
            match i {
                0 => write!(f, "0: {}", el)?,
                i => write!(f, ", {index}: {value}", index = i, value = el)?,
            };
        }

        write!(f, " ]")
    }
}
