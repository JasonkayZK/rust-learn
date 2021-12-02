trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("from origin")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("from trait")
    }
}

fn main() {
    println!("a baby dog name: {}", Dog::baby_name()); // Default use origin func

    // Compiling err: cannot infer type
    // println!("a baby dog name: {}", Animal::baby_name());

    // fully qualified
    println!("a baby dog name: {}", <Dog as Animal>::baby_name());
}
