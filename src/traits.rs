use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("Tweet from {}", self.summarize_author())
    }
}

// other version: fn notify(item: impl Summary + Display) {}
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// sugar syntax
// other version: fn notify_sugar<T: Summary + Display>(item: T) {}
fn notify_sugar<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// where version
fn notify_where<T>(item: T) -> () where T: Summary {
    println!("Breaking news! {}", item.summarize());
}

// return `impl Trait` return generic in short
// warning: it makes your code hard to predict which type would be return if call
// only use in some specific case such as libs expose api, the context of closures and iterators
// remember: func should accept abstract but return concrete
fn make_notifiable_news() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // return unpredictable struct is not allowed here since Rust compiler must know
    // which type will be returned at compile time :)
}


pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = Article {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    notify(article);
    notify_sugar(tweet);
}