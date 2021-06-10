extern crate generic_trait_lifetime;

use generic_trait_lifetime::{Tweet, Summarizable, notify};

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("high: {}, low: {}, chance of precipitation: {}",
                self.high_temp,
                self.low_temp,
                self.chance_of_precipitation,
        )
    }
}

struct DefaultSummary{}

impl Summarizable for DefaultSummary {
    fn summary(&self) -> String {
        String::from("a default summary")
    }
}

fn main() {
    trait_demo1();
    trait_demo2();
    default_trait_demo();
    another_default_trait_demo();

    trait_bounds_demo();
}

fn trait_demo1() {
    let tweet = Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you know"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());
}

fn trait_demo2() {
    println!("the weather report: {}", WeatherForecast {
        high_temp: 30.0,
        low_temp: 20.0,
        chance_of_precipitation: 0.55
    }.summary())
}

fn default_trait_demo() {
    println!("summary: {}", DefaultSummary{}.default_summary());
}

fn another_default_trait_demo() {
    println!("summary: {}", DefaultSummary{}.another_summary());
}

fn trait_bounds_demo() {
    notify(DefaultSummary{});
    notify(WeatherForecast {
        high_temp: 30.0,
        low_temp: 20.0,
        chance_of_precipitation: 0.55
    });
    notify(Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you know"),
        reply: false,
        retweet: false,
    });

    // Compiling error! String has no `summary` method！
    // notify(String::from("wrong!"));
}
