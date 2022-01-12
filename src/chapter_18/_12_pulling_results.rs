pub fn run() {
    first();
    second();
}

fn first() {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}

fn second() {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

        opt.map_or(Ok(None), |r| r.map(Some))
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
