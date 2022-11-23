use chrono::Local;

fn main() {
    let date = Local::now();
    println!("{}", date.format("[%Y-%m-%d] [%H:%M:%S]"));
}
