fn main() {
//    loop {
//        println!("again!");
//    }
    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("End");

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is {element}");
    }
    
    // Range
    for i in 0..3 {
        println!("{i}");
    }

    for i in (0..3).rev() {
        println!("{i}");
    }

    for i in 'a'..'d' {
        println!("{i}");
    }

    for i in -10..20 {
        println!("{i}");
    }
}
