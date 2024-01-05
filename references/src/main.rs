fn get_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" you");
}

/*
fn dangle() -> &String {
    let s = String::from("Out");
    &s // DANGER!!!
}
*/

fn main() {
    let mut s1  = String::from("hey");
    let length = get_length(&s1); 
    println!("{} {}", s1, length);

    change(&mut s1);
    println!("{}", s1);
}
