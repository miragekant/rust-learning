fn main() {
    let s = String::from("hey you");
    println!("{}", first_word(&s));

    let s2 = String::from("hey you abc");
    let first = &s2[0..5];
    let second = &s2[5..8];

    println!("{} {}", first, second);

    // Identical
    let s = String::from("Hello");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let s = String::from("Hello");
    let length = s.len();
    let slice = &s[3..];
    println!("{}", slice);
    let slice = &s[3..length];
    println!("{}", slice);

    let s = String::from("a b c");
    let word = first_word_ex(&s);
    // immutable borrow later used
    //s.clear();
    println!("{}", word);
    let word2 = first_word_new(&s);
    println!("{}", word2);
    
    // String literals are slices already
    // Equivalent
    let str_l = "well well";
    let word = first_word_new(str_l);
    println!("{}", word);
    let word = first_word_new(&str_l);
    println!("{}", word);
    let word = first_word_new(&str_l[..]);
    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_ex(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_new(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
// fn second_word(s: &String) -> (usize, usize) { MORE TROUBLE


