use std::io;

fn main() {
    const ONE_HOUR_IN_SECONDS: u32 = 3600;
    println!("One hour in seconds: {ONE_HOUR_IN_SECONDS}"); 

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 1;
    let y = y + 1;
    // Inner scope
    {
        let y = y + 3;
        println!("y: {y}");
    }
    // Back to outer scope
    println!("y: {y}");


    // Changing types
    let spaces = "  ";
    let spaces = spaces.len();
    
    // Visual separators
    let mut number: i32 = 98_222;
    number = 0b1111_0000;
    println!("{number}");

    // Float
    let f = 2.0;
    let fa: f32 = 1.0;
    println!("{f}");
    println!("{fa}");

    // Other types
    let t: bool = true;
    // UTF-8
    let c: char = '你';
    println!("{t}, {c}");


    // Tuple
    let t: (i32, f64, u8) = (500, 12.0, 1);
    //println!("{t.0}, {t.1}, {t.2}");
    let (x, y, z) = t;
    println!("{x}, {y}, {z}");
    
    // Array
    let a = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered ws not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
