struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

/*
Result:
    CustomSmartPointers created
    Dropping CustomSmartPointer with data `other stuff`
    Dropping CustomSmartPointer with data `my stuff`

The variable will be released reversely against to its creating sequence.
 */
fn drop_demo() {
    let a = CustomSmartPointer { data: String::from("my stuff") };
    let b = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created");
}

fn early_drop_demo() {
    let x =CustomSmartPointer { data: String::from("my stuff") };
    println!("CustomSmartPointer created");

    // Explicit use destructor is not allowed!
    // x.drop();

    std::mem::drop(x);
    println!("CustomSmartPointer dropped before the end of main");
}

fn main() {
    drop_demo();
    early_drop_demo();
}
