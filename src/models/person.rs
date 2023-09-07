#[derive(Debug, Default, PartialEq)]
pub struct Person {
    pub uid: String,
    pub name: String,
    pub age: i32,
    pub info: String,
    pub gender: i32,
    pub create_time: i32,
}

pub enum GenderEnum {
    Unknown,
    Man,
    Woman,
}

impl From<GenderEnum> for i32 {
    fn from(value: GenderEnum) -> Self {
        match value {
            GenderEnum::Unknown => 0,
            GenderEnum::Man => 1,
            GenderEnum::Woman => 2,
        }
    }
}

#[cfg(test)]
mod tests {}
