use collections_exercise::{ex1,ex2,ex3};
use std::collections::HashMap;

fn main() {
    // ex1
    println!("Exercise 1:");
    let v = vec![30, 28, 59, 3, -100, -100];
    let m = ex1::median(&v);
    let mo = ex1::mode(&v);
    println!("List of integers: {:?}, median: {}, mode: {}", v, m, mo);
    
    // ex2
    println!("\nExercise 2:");
    let text1 = "banana";
    let text2 = "apple";
    println!("Converting {} to {}", text1, ex2::string_to_pig_latin(text1));
    println!("Converting {} to {}", text2, ex2::string_to_pig_latin(text2));

    // ex3
    println!("\nExercise 3:");
    let mut company = HashMap::new();
    ex3::add_employee_to_department(&mut company, "Engineering", "Amir");
    ex3::add_employee_to_department(&mut company, "Engineering", "Robert");
    ex3::add_employee_to_department(&mut company, "Engineering", "Weslie");
    ex3::add_employee_to_department(&mut company, "Sales", "Charles");
    ex3::add_employee_to_department(&mut company, "Sales", "Becca");
    ex3::add_employee_to_department(&mut company, "Operation", "Sally");
    ex3::show_employees(&company);
}
