fn main() {
    another_function(5);
    other_function(5, 6);

    let y = {
        let x = 3;

        // if fllowing line has semicolon,
        // it becomes "sentense"
        x + 1
    };

    println!("The value of y is: {}", y);

    let five = five();
    println!("The value of five is: {}", five);

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);

    // Following block is expression
}

fn other_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
