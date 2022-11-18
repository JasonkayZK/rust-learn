use anyhow::Result;
use std::fs;

fn read_file() -> Result<i64> {
    // Could get compiled!
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<()> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
