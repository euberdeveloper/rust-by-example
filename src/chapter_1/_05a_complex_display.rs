use std::fmt::{self, Display, Formatter};

pub fn run() {
    let n = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("The complex number is {}", n);
    println!("{0:?} {:#?}", n);
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} + {}i)", self.real, self.imag)
    }
}
