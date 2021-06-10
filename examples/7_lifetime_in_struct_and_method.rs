// A struct has refer elem
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime in method
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Inferred lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // No need to add lifetime, give out the ownership!
    fn show_len(&self, str1: &str, str2: &str) -> String {
        format!("self len: {}, str1 len: {}, str2 len: {}",
                self.part.len(),
            str1.len(),
            str2.len()
        )
    }

    // Explicit lifetime required!
    fn get_compare(&'a self, str1: &'a str, str2: &'a str) -> &'a str {
        if self.part.starts_with("longest") {
            if str1.len() >= str2.len() {
                str1
            } else {
                str2
            }
        } else if self.part.starts_with("shortest") {
            if str1.len() >= str2.len() {
                str2
            } else {
                str1
            }
        } else {
            self.part
        }
    }
}

fn main() {
    // Lifetime in struct elem
    let novel = String::from("Call me jasonkay. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("i: {:?}", i);

    // Lifetime in method
    println!("level: {}", i.level());
    println!("announce_and_return_part: {}", i.announce_and_return_part("hello?"));

    println!("show_len: {}", i.show_len("abc", "abcd"));

    let longest = ImportantExcerpt {part: "longest"};
    println!("longest: {}", longest.get_compare("a", "abcd"));
    let shortest = ImportantExcerpt {part: "shortest"};
    println!("shortest: {}", shortest.get_compare("a", "abcd"));
    let other = ImportantExcerpt {part: "other"};
    println!("other: {}", other.get_compare("a", "abcd"));
}
