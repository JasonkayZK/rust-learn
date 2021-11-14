fn match_with_ref() {
    let robot_name = Some(String::from("Britz"));

    match robot_name {
        // Compiling err: value has been moved!
        // Some(name) => println!("Found a name: {}", name),
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    };

    println!("robot_name is: {:?}", robot_name);
}

fn match_with_mut_ref() {
    let mut robot_name = Some(String::from("Britz"));

    match robot_name {
        // Compiling err: value has been moved!
        // Some(name) => println!("Found a name: {}", name),
        Some(ref mut name) => *name = String::from("another name"),
        None => (),
    };

    println!("robot_name is: {:?}", robot_name);
}

fn main() {
    match_with_ref();

    match_with_mut_ref();
}
