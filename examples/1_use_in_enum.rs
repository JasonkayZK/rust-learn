#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    use_demo1();

    glob_demo();
}

fn use_demo1() {
    use TrafficLight::{Red, Yellow};

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("red: {:?}, yellow: {:?}, green: {:?}", red, yellow, green);
}

fn glob_demo() {
    use TrafficLight::*;
    let red = Red;
    let yellow = Yellow;
    let green = Green;

    println!("red: {:?}, yellow: {:?}, green: {:?}", red, yellow, green);
}
