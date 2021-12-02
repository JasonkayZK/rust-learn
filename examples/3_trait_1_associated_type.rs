// Announce a associated-type
pub trait Iterator {
    type Item;
    // Announce a associated-type
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    list: Vec<i64>,
}

// Implement an associated-type trait
impl Iterator for Counter {
    type Item = i64; // Type Must be explict!

    fn next(&mut self) -> Option<Self::Item> {
        return self.list.pop();
    }
}

/* Compiling err: conflicting implementation for `Counter`!

impl Iterator for Counter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        return self.list.pop();
    }
}
*/

pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct AnotherCounter<T> {
    list: Vec<T>,
}

impl GenericIterator<i64> for AnotherCounter<i64> {
    // explict announce type!

    fn next(&mut self) -> Option<i64> {
        self.list.pop()
    }
}

// Ok to compiling!
impl GenericIterator<f64> for AnotherCounter<f64> {
    // explict announce type!
    fn next(&mut self) -> Option<f64> {
        self.list.pop()
    }
}

fn main() {
    println!(
        "{}",
        Counter {
            list: vec![1, 2, 3]
        }
        .next()
        .unwrap()
    );
    println!(
        "{}",
        AnotherCounter {
            list: vec![1, 2, 3]
        }
        .next()
        .unwrap()
    );
    println!(
        "{}",
        AnotherCounter {
            list: vec![1.1, 2.2, 3.3]
        }
        .next()
        .unwrap()
    );

    // Compiling err: method `next` not found for this!
    // println!("{}", AnotherCounter { list: vec!["1", "2", "3"] }.next().unwrap());
}
