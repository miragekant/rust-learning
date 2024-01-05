mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches");
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

// Not working
mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// Working
mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        // or
        //super::hosting::add_to_waitlist();
    }
}

// It's Idiomatic to specify the full path
use std::collection::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Providing new names with the as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// re-exporting
pub use crate::front_of_house::hosting;
// External code can use
restaurant::hosting::add_to_waitlist();
// Instead of
restaurant::front_of_house::add_waitlist();

// Nested paths
use std::cmp::Ordering;
use std::io;
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
use std::io::{self, Write};

// Glob operator
use std::collections::*;
