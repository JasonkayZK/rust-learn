fn default_generic<T: Sized>(_: T) {}

fn unsized_generic<T: ?Sized>(_: &T) {}

fn main() {
    // Compiling err:
    // let s1: str = "Hello";

    let _: &str = "Hello";
}
