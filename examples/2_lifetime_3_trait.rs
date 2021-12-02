use std::fmt::Debug;

trait Red {}

#[derive(Debug)]
struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

impl Debug for dyn Red {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Red")
    }
}

fn main() {
    let n = 5;

    let obj = Box::new(Ball { diameter: &n }) as Box<dyn Red>;

    // Compiling err if added line below:
    // println!("{:?}", obj);

    // To fix, use this:
    // let n: &'static i32 = &5;

    // More info, see: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
}
