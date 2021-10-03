use my_libs::hello::say_hello;

fn main() {
    match say_hello("jasonkay") {
        Ok(res) => {
            if res { println!("let's rust-up!") }
        }
        Err(e) => { println!("error occurred: {:?}", e) }
    }
}
