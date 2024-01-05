fn add(x: i32, y: i32) -> i32 {
    x + y
}


fn main() {
    // Statements evaluate to ()
    let result = add(10, 20);
    println!("{result}");

    if result > 10 {
        println!("Yes");
    } else {
        println!("No");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2"); 
    }

    // if is an expression 
    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
    let number = if true { 5 } else { 6 };
    println!("{number}");
}
