trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("fly from human origin");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("fly from pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("fly from wizard");
    }
}

fn main() {
    let person = Human;
    person.fly(); // default calls its own method!
    Human::fly(&person); // call own method explicitly

    Pilot::fly(&person);
    Wizard::fly(&person);
}
