use std::fmt::{self, Display, Formatter};

pub fn run() {
    let c1 = Colour(128, 255, 90);
    let c2 = Colour(0, 3, 254);
    let c3 = Colour(0, 0, 0);

    println!("c1 is {}", c1);
    println!("c2 is {}", c2);
    println!("c3 is {}", c3);
}

struct Colour(u8, u8, u8);

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Colour(red, green, blue) = self;
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:0>2X}{g:0>2X}{b:0>2X}",
            r = red,
            g = green,
            b = blue
        )
    }
}
