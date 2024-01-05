fn main() {
    let mut s = String::new();
    
    // to_string method available for Display trait
    let data = "initial contents";
    let s = data.to_string();
    // also works
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    println!("{}", s); 

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // string slice
    println!("{}, {}", s1, s2);
    
    // Concat with + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    // Strings not support indexing directly 
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
}
