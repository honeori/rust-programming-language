fn main() {
    {
        let mut s = String::from("hello world");
        let _word = first_word_not_good(&s);

        s.clear();

        // s is empty, but the value of word is , fmm
    }

}

fn first_word_not_good(s: &String) -> usize {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
