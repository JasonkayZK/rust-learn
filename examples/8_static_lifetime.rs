extern crate rand;

use rand::{Rng};
use rand::distributions::Alphanumeric;

fn main() {
    // 'static is ok, because the two param is const!
    println!("static_lifetime_demo: {}", static_lifetime_demo("ab", "abcd"));

    for i in 0..10 {
        // Cause err: creates a temporary which is freed while still in use
        // println!("{}-th longest: {}", i, static_lifetime_demo(get_random_string(50).as_str(), get_random_string(40).as_str()));
        println!("{}-th longest: {}", i,
                 fix_static_lifetime_demo(
                     get_random_string(50).as_str(),
                     get_random_string(40).as_str())
        );
    }
    // println!("{}, {}", get_random_string(50), get_random_string(40));
}

// Use lots of memory, since all str will be stored!
fn static_lifetime_demo(str1: &'static str, str2: &'static str) -> &'static str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn fix_static_lifetime_demo<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
