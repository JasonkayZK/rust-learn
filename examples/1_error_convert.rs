#[derive(Debug)]
pub enum MyError {
    ReadError(String),
    ParseError(String),
}

fn read_file() -> Result<i64, MyError> {
    // Error: Could not get compiled!
    // let content = fs::read_to_string("/tmp/id")?;
    // let id = content.parse::<i64>()?;

    // Method 1: Handling error explicitly!
    let content = match std::fs::read_to_string("/tmp/id") {
        Ok(content) => content,
        Err(err) => {
            return Err(MyError::ReadError(format!("read /tmp/id failed: {}", err)));
        }
    };
    let content = content.trim();
    println!("read content: {}", content);

    // Method 2: Use map_err to transform error type
    let id = content
        .parse::<i64>()
        .map_err(|err| MyError::ParseError(format!("parse error: {}", err)))?;

    Ok(id)
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
