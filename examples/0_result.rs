#[derive(Debug)]
pub enum MyError {
    Internal(String),
    InvalidId(String),
}

fn add(num: i64) -> Result<i64, MyError> {
    if num < 0 {
        Err(MyError::InvalidId(String::from("Invalid num!")))
    } else {
        Ok(num + 100000)
    }
}

fn main() -> Result<(), MyError> {
    // fetch_id(-1)?;

    let res = add(1)?;
    println!("{}", res);
    Ok(())
}
