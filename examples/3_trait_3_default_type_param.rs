use std::ops::Add;

#[derive(Debug)]
struct MilliMeters(u32);
#[derive(Debug)]
struct Meters(u32);

// Announce type Meters explicitly to avoid default: Add<Rhs = Self>
impl Add<Meters> for MilliMeters {
    type Output = MilliMeters;

    fn add(self, rhs: Meters) -> Self::Output {
        MilliMeters(self.0 + (rhs.0 * 1000))
    }
}

fn main() {
    println!("{:?}", MilliMeters(12) + Meters(1))
}
