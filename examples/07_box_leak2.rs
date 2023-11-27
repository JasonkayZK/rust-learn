#[derive(Debug)]
struct Config {
    _a: String,
    _b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        _a: "A".to_string(),
        _b: "B".to_string(),
    });

    Some(Box::leak(c))
}


fn main() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}
