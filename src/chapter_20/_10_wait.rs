use std::process::Command;

pub fn run() {
    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
