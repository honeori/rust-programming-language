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
