#[derive(Debug)]
struct Rectrangle {
    width: u32,
    height: u32,
}

impl Rectrangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectrangle {width: 30, height: 50};

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


