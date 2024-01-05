// Vectors with enum
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Macro
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third element: {}", third);

    // get method returns an option
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("No third element"),
    }

    // panic
    // let does_not_exist = &v[100];
    
    let does_not_exist = v.get(100);
    if let None = does_not_exist {
        println!("Not exist");
    } else {
        println!("Exist");
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &mut v[0];
    *first = 100;
    println!("{}", first);


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", *i);
    }
    
    let row = vec! {
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.12),
    };
}

