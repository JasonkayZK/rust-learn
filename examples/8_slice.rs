fn main() {
    first_word_idx_demo();

    slice_demo();

    first_word_slice_demo();
}

fn first_word_idx_demo() {
    let mut s = String::from("hello world");

    // get index from word
    let word_idx = first_word_idx(&s);
    println!("word_idx: {}", word_idx);

    // Use s.clear() to clear the info in string s
    s.clear();

    // Will cause err if use word_idx to traverse s!
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    };

    s.len()
}

fn slice_demo() {
    let s = String::from("hello world");
    let s1 = &s[..5]; // Equals to &s[0..5]
    let s2 = &s[6..]; // Equals to &s[6..11]
    let s3 = &s[..]; // Equals to &s[0 .. s.len()];

    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
}

fn first_word_slice_demo() {
    let mut s = String::from("hello world");

    // get index from word
    let word_slice = first_word_slice(&s);
    println!("word_slice: {}", word_slice);

    s.clear();

    // We can't use word_slice here, because `s.clear()` is a mutable borrow from `s`!
    // let again = first_word_slice(word_slice);
    // println!("again: {}", again);

    // But we can use `s` here
    if s.len() > 0 {
        let (second_slice, exist) = second_word_slice(&s);
        if exist {
            println!("second: {}", second_slice);
        }
    } else {
        println!("s is empty!");
    }
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    };

    &s[..]
}

fn second_word_slice(s: &str) -> (&str, bool) {
    let bytes = s.as_bytes();
    let mut seen = false;
    let mut prev = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if seen {
                return (&s[prev + 1..i], true);
            } else {
                seen = true;
                prev = i;
            }
        }
    };

    println!("prev: {}, seen: {}", prev, seen);

    return if seen {
        (&s[prev + 1..], true)
    } else {
        ("", false)
    }
}
