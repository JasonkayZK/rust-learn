use boost_rs::types::as_any::{AsAny, Downcast};

trait Custom: AsAny {
    fn hello(&self) -> String;
}

struct Test {
    age: i32,
}

impl Custom for Test {
    fn hello(&self) -> String {
        String::from("This is Test!")
    }
}

fn main() {
    let x: Box<dyn Custom> = Box::new(Test { age: 1 });
    // Wrong:
    // println!("age: {}", x.downcast_ref::<Test>().unwrap().age);
    println!("age: {}", (*x).downcast_ref::<Test>().unwrap().age);

    let y: &dyn Custom = &Test { age: 2};
    println!("age: {}", y.downcast_ref::<Test>().unwrap().age)
}
