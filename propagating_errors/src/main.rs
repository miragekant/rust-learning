use std::fs::{self. File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    */
    // ? operator
    // if Ok, get result. Otherwise, return Err
    /*
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_str(&mut username)?;
    Ok(username)
    */

    // Chaining
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Even shorter
    //fs::read_to_string("hello.txt")
}

// ? operator also works on Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
   read_username_from_file(); 
}

// main() can also return Result<(), E>
// Box<dyn Error> type is a trait object
// It allows any kinds of errors
/*
use std::error::Errpr;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(());
}
*/
