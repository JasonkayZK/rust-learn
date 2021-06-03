fn main() {
    let my_str = String::from("hello world");
    let word = first_word(&my_str[..]);
    println!("word: {}", word);
    let word2 = first_word(&my_str);
    println!("word2: {}", word2);

    let my_str_literal = "hello world";
    let word = first_word(&my_str_literal[..]);
    println!("word: {}", word);
    let word2 = first_word(&my_str_literal);
    println!("word2: {}", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    };

    &s[..]
}
