// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {
    Bar,
    Buzz,
}

pub fn run() {
    let a = Foo::Bar;
    let b = Foo::Buzz;

    // Variable a matches Foo::Bar
    // if Foo::Bar == a {
    if let Foo::Bar = a {
        // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    } else if let Foo::Buzz = a {
        println!("a is a buzz");
    }

    if let Foo::Bar = b {
        // ^-- this causes a compile-time error. Use `if let` instead.
        println!("b is foobar");
    } else if let Foo::Buzz = b {
        println!("b is b buzz");
    }
}
