use std::fs;

#[derive(Debug)]
pub enum MyError {
    ReadError(String),
    ParseError(String),
}

impl From<std::io::Error> for MyError {
    fn from(source: std::io::Error) -> Self {
        MyError::ReadError(source.to_string())
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(source: std::num::ParseIntError) -> Self {
        MyError::ParseError(source.to_string())
    }
}

fn read_file() -> Result<i64, MyError> {
    let _content = fs::read_to_string("/tmp/id")?;
    let content = _content.trim();
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
