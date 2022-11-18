use std::fs;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("io error.")]
    IoError(#[from] std::io::Error),
    #[error("parse error.")]
    ParseError(#[from] std::num::ParseIntError),
}

fn read_file() -> Result<i64, MyError> {
    // Could get compiled!
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
