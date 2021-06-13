use std::fs::File;
use std::io::prelude::*; // import prelude in std::io!

fn main() {
    let filename = "poem.txt";
    println!("filename: {}", filename);

    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("file content:\n{}", contents);
}