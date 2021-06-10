// Define Trait
pub trait Summarizable {
    fn summary(&self) -> String;

    // Default trait method
    fn default_summary(&self) -> String {
        String::from("(read more...)")
    }

    // Another default trait method to use other trait method
    fn another_summary(&self) -> String {
        format!("(Read more from {}...)", self.summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait bounds!
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
