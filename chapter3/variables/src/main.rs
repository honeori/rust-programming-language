fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of shadowing x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // Following part will can not be compiled
    // mismatched types error will occur
    /*
    let mut spaces = "    ";
    spaces = spaces.len();
    */

}
