pub fn run() {
    first();
    second();
    third();
    fourth();
}

fn first() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}

fn second() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
//     println!("Results: {:?}", numbers);
}

fn third() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn fourth() {
    // let strings = vec!["tofu", "93", "18"];
    // let (numbers, errors): (Vec<_>, Vec<_>) = strings
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .partition(Result::is_ok);
    // let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    // let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    // println!("Numbers: {:?}", numbers);
    // println!("Errors: {:?}", errors);
}
