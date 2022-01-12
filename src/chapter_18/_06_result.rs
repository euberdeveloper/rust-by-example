use std::num::ParseIntError;

fn fun() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}


pub fn run() {
    if let Err(e) = fun() {
        println!("Error {}", e);
    }
    else {
        println!("Alles gut");
    }
}