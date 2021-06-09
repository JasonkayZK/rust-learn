use std::fs::File;
use std::io::{ErrorKind};

fn main() {
    result_demo1();
    result_demo2();
}

fn result_demo1() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => { file }
        Err(err) => {
            panic!("Err open file: {:?}", err)
        }
    };
}

fn result_demo2() {
    let f = File::open("hello2.txt");

    let _f = match f {
        Ok(file) => { file }

        // Match guard
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Error creating file: {:?}", e)
                }
            }
        }
        
        Err(error) => {
            panic!("Error opening file: {:?}", error);
        }
    };
}
