use std::fmt::Display;

// Borrow checker
// This won't work
/*
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
*/

/*
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
*/

// This won't work
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// if we always return the first param
/*
fn longest<'a>(x: &'a str, y: &str> -> &'a str {
    x
}
*/


// Lifetime elision rules
// 1. The compiler assigns a lifetime parameter to each parameter that's a reference.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

// Examples:
// fn first_word(s: &str) -> &str {}
// Apply first rule:
// fn first_word<'a>(s: &'a str) -> &str {}
// Apply second rule:
// fn first_word<'a>(s: &'a str) -> &'a str {}
// Compiler can continue analysis without needing to annotate the lifetimes in function signature

// fn longest(x: &str, y: &str) -> &str {}
// Apply first rule:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// Second rule does not apply

// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Lifetime parameter declaration after impl and its use after the type name are required, but not required to annotate the lifetime of the reference to self
    fn level(&self) -> i32 {
        3
    }
    
    // Return type gets the lifetime of &self
    fn announcement_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Combining generic type parameters, trait bounds and lifetimes together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    longest_with_an_announcement("abc", "Big fat fox", "Wow!!!");

    // Static lifetime
    let s: &'static str = "I have a static lifetime."; 

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };


    let string1 = String::from("abcdd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // String 2 does not live long enough
    /*
    let string1 = String::from("abcdd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
    */
}
