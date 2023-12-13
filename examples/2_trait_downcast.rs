use std::any::Any;

#[derive(Default)]
struct Test {
    age: i32,
}

trait Custom: AsAny {
    fn hello(&self) -> String;
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl AsAny for Test {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Custom for Test {
    fn hello(&self) -> String {
        String::from("hello")
    }
}

fn main() {
    let test = Test { age: 1 };
    let custom: Box<dyn Custom> = Box::new(test);
    println!(
        "age: {}",
        custom.as_any().downcast_ref::<Test>().unwrap().age
    )
}
