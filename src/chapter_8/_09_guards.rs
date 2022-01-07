pub fn run() {
    let pair = (2, -2);
    // ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        // Note that the compiler does not check arbitrary expressions for whether all possible 
        // conditions have been checked. Therefore, you must use the _ pattern at the end.
        _ => println!("No correlation..."),
    }
}
