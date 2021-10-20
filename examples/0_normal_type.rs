use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
struct Meter {
    value: f64,
}

impl Meter {
    fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Add for Meter {
    type Output = Meter;

    fn add(self, another: Meter) -> Self::Output {
        let value = self.value + another.value;
        Meter { value }
    }
}

impl Sub for Meter {
    type Output = Meter;

    fn sub(self, another: Meter) -> Self::Output {
        let value = self.value - another.value;
        Meter { value }
    }
}

fn main() {
    let one = Meter::new(1.1);
    let three = Meter::new(3.3);

    let four = one + three;
    dbg!(&four);

    let two = three - one;
    dbg!(&two);
}
