extern crate core;

use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// Closure in struct
struct Cacher<T, I, O> where T: Fn(I) -> O {
    algorithm: T,
    value: HashMap<I, O>,
}

impl<T, I, O> Cacher<T, I, O>
    where T: Fn(I) -> O,
          O: Copy,
          I: Copy + Eq + Hash {
    fn new(calculation: T) -> Cacher<T, I, O> {
        Cacher {
            algorithm: calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> O {
        match self.value.get(&arg) {
            None => {
                let v = (self.algorithm)(arg);
                self.value.insert(arg.clone(), v);
                v
            }
            Some(v) => { v.clone() }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_closure.value(intensity),
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_closure.value(intensity),
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            )
        }
    }
}

fn closure_demo1() {
    let simulated_user_specified_val = 10;
    let simulated_random_num = 7;

    generate_workout(
        simulated_user_specified_val,
        simulated_random_num,
    );
}

fn closure_type_infer() {
    let mirror = |x| x;

    println!("mirror: {}", mirror(5));

    // Cause err: mirror already int type!
    // println!("mirror: {}", mirror(String::from("hello")));
}

fn generic_type_in_closure() {
    let mut c = Cacher::new(|x| {
        println!("this is: {:?}", x);
        // Could return any type!
        1.222
    });
    let v1 = c.value(1);
    let v2 = c.value(2);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

fn closure_catch_env() {
    let x = 4;
    let closure = |z| z == x;
    let y = 4;
    println!("closure: x == y? {}", closure(y));

    // Compiling err: Closure ONLY!
    // fn func(z: i32) -> bool {z == x};
    // println!("func: x == y? {}", func(y));
}

fn main() {
    closure_demo1();

    closure_type_infer();

    generic_type_in_closure();

    closure_catch_env();
}

#[test]
fn call_with_different_val() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(1, v1);
    assert_eq!(2, v2);
}
