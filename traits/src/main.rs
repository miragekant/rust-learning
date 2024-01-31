use aggregator::{Summary, NewsArticle, Tweet};
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

// Conditionally implements methods
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is x = {}", self.y);
        }
    }
}

// Implementing a trait for any types that implements another trait
// called blanket implementation
// Implements ToString trait for all types that implement Display trait
/*
impl<T: Display> ToString for T {
    // --snip--
}
*/

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
/*
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/

//fn notify(item1: &impl Summary, item2: &impl Summary) {}
// If we want item1 and item2 to be the same type strictly 
//fn notify<T: Summary>(item1: &T, item2: &T) {}

// Multiple trait bounds
//fn notify(item: &(impl Summary + Display)) {}
//fn notify<T: Summary + Display>(item: &T) {}   
// With where clause
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
*/

// Returning types that implement traits
// Can only use when returning a single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburge, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are thee best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    
    notify(&tweet);

    let s = 3.to_string();
    println!("{s}");
}
