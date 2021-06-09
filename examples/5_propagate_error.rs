use std::io;
use std::fs::File;
use std::io::{Read};

fn main() {
    // read_username_from_file().unwrap();
    read_username_from_file2().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("not_found.txt");

    let mut f = match f {
        Ok(file) => { file }
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => { Ok(s) }
        Err(e) => { Err(e) }
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    /*
    let mut f = File::open("not_found.txt")?;
    f.read_to_string(&mut s)?;
    */

    // In one line:
    File::open("not_found.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
