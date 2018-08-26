fn main() {
    move_ownership();
    clone_and_preserve_ownership();
    
    let s1 = String::from("Hello");
    // move owenership
    taken_ownership(s1);
    
    // use of moved value error will be occur
    //println!("{}", s1);
   
    let s2 = String::from("s2 Hello");
    let s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    {
        let s = String::from("hello");
        let (_s, _length) = calculate_length_annoying(s);
    }
    {
        let s = String::from("hello");
        // not move ownership by using reference
        // it called `borrowing`
        let _length = calculate_length(&s);
    }
        
}

fn move_ownership() {
    let s1 = String::from("hello");
    let _s2 = s1;

    // use of moved value: `s1` error will occur
    //
    //println!("{}, world", s1);
}

fn clone_and_preserve_ownership() {
    let s1 = String::from("hello");
    // s1 preseve ownership
    let _s2 = s1.clone();

    // error does not occur
    println!("{}, world", s1);
}

fn taken_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length_annoying(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// compile error occuer: missing lifetime specifier
/*
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
*/
