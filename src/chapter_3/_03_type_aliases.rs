#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the Self alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    #[allow(dead_code)]
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub fn run() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let _x = Operations::Add;
}
