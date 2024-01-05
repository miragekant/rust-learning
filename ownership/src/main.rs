fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn main() {
    let _s = "Hello";

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);


    let s = String::from("Hello");

    takes_ownership(s);
    // Won't work
    //println!("{}", s);

    let n = 5;
    makes_copy(n);

    let s1 = gives_ownership();
    let s2 = String::from("hey");
    let s3 = takes_and_gives_back(s2);
    
    println!("{}, {}", s1, s3);
}
