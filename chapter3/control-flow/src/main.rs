fn main() {
    if_keyword();
    loop_keyword();
    while_keyword();
    for_keyword();

    let fib5 = fibonacci(10);
    println!("value of fib5 is: {}", fib5);
}

fn if_keyword() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // In following condition( given not boolean v), error will happen
    /*
    if number {
        println!("condition was i32");
    }
    */

    // "if" is experession, so following block is right.
    let condition = true;
    let number = if condition {
        3
    } else {
        4
    };

    println!("The value of number is {}", number);
}

fn loop_keyword() {
    // folowing block will occur infinit loop
    /*
    loop {
        println!("again!");
    }
    */
}

fn while_keyword() {
    let mut number = 3;
    while number != 0 {
        println!("value of number is: {}", number);
        number = number - 1;
    }
}


fn for_keyword() {
    let a = [10, 20, 30, 40 ,50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in (1..4).rev() {
        println!("the value is: {}", element);
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
