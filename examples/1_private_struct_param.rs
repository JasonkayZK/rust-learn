use crate::my_collection::AverageCollection;

mod my_collection {
    pub struct AverageCollection {
        // No `pub`: private param
        list: Vec<i32>,
        average: f64,
    }

    impl AverageCollection {
        /* impl some pub method */

        pub fn new() -> Self {
            AverageCollection {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average(value, 1);
        }

        pub fn remove(&mut self) -> Option<i32> {
            let res = self.list.pop();
            match res {
                Some(val) => {
                    self.update_average(val, -1);
                    Some(val)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        /* private method */

        fn update_average(&mut self, offset: i32, opt_flag: i32) {
            match self.list.len() {
                0 => self.average = 0.0,
                _ => {
                    self.average = ((self.average * (self.list.len() as f64 - opt_flag as f64))
                        + (opt_flag * offset) as f64)
                        / (self.list.len() as f64)
                }
            }
        }
    }
}

fn main() {
    let mut col = AverageCollection::new();

    // private field: can't visit directly!
    // println!("{}", col.list.len());

    col.add(12);
    println!("average: {}", col.average());

    col.add(33);
    println!("average: {}", col.average());

    col.remove();
    println!("average: {}", col.average());

    col.remove();
    println!("average: {}", col.average());
}
