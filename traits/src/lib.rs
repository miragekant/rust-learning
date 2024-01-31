pub trait Summary {
    //fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        //String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    /*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    */
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
