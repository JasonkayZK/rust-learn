use std::any::{Any, TypeId};

#[derive(Default)]
struct Test1 {
    age: i32,
}

#[derive(Default)]
struct Test2 {
    salary: i32,
}

trait Custom: AsAny {
    fn hello(&self) -> String;
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn type_id(&self) -> TypeId;
}

impl AsAny for Test1 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl Custom for Test1 {
    fn hello(&self) -> String {
        String::from("hello from test1")
    }
}

impl AsAny for Test2 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl Custom for Test2 {
    fn hello(&self) -> String {
        String::from("hello from test2")
    }
}

fn main() {
    let mut v: Vec<Box<dyn Custom>> = vec![];
    let test1 = Box::new(Test1 { age: 1 });
    let test2 = Box::new(Test2 { salary: 2 });
    v.push(test1);
    v.push(test2);

    for item in v {
        println!("{}", item.hello());
        let any_item = item.as_any();

        if any_item.type_id() == TypeId::of::<Test1>() {
            println!("age: {}", any_item.downcast_ref::<Test1>().unwrap().age)
        }
        if any_item.type_id() == TypeId::of::<Test2>() {
            println!(
                "salary: {}",
                any_item.downcast_ref::<Test2>().unwrap().salary
            )
        }
    }
}
