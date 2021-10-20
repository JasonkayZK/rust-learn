use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
struct Unit<T> {
    value: f64,
    unit_type: T,
}

impl<T: Default> Unit<T> {
    fn new(value: f64) -> Self {
        Self {
            value,
            unit_type: T::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MeterType;
type Meter = Unit<MeterType>;

#[derive(Debug, Copy, Clone)]
struct KilogramType;
type Kilogram = Unit<KilogramType>;

impl Default for MeterType {
    fn default() -> Self {
        MeterType
    }
}

impl Default for KilogramType {
    fn default() -> Self {
        KilogramType
    }
}

impl<T: Default> Add for Unit<T> {
    type Output = Unit<T>;

    fn add(self, another: Unit<T>) -> Self::Output {
        let new_value = self.value + another.value;
        Unit::new(new_value)
    }
}

impl<T: Default> Sub for Unit<T> {
    type Output = Unit<T>;

    fn sub(self, another: Unit<T>) -> Self::Output {
        let new_value = self.value - another.value;
        Unit::new(new_value)
    }
}

fn main() {
    let one = Meter::new(1.1);
    let three = Meter::new(3.3);

    let four = one + three;
    dbg!(&four);

    let one = Kilogram::new(1.1);
    let three = Kilogram::new(3.3);

    let four = one + three;
    dbg!(&four);

    // Compiling err!

    // let one = Meter::new(1.1);
    // let three = Kilogram::new(3.3);

    // let four = one + three;
    // dbg!(&four);
}
