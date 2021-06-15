//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::SecondaryColor::Orange;
    use crate::kinds::{PrimaryColor, SecondaryColor};

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!("c1: {:?}, c2: {:?}", c1, c2);
        Orange
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        println!("hello")
    }
}
