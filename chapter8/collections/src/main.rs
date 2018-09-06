fn main() {
    let mut v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3]; initialized with values
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    //let out_of_index: &i32 = &v[100]; panic occurred
    let out_of_index: Option<&i32> = v.get(100);  // None

    iterate();

}

fn iterate() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
}
