struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();

    println!("{}",counter.next() == Some(1));
    println!("{}",counter.next() == Some(2));
    println!("{}",counter.next() == Some(3));
    println!("{}",counter.next() == Some(4));
    println!("{}",counter.next() == Some(5));
    println!("{}",counter.next() == None);
    println!("{}",counter.next() == None);
}

fn other_func() {
    let sum : u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("res: {}", 18 == sum);
}

fn main() {
    calling_next_directly();

    other_func();
}
