use std::convert::TryFrom;
use std::fmt::Display;

use crate::sync::models::OpEnum::{Delete, Insert, Update};

#[derive(Debug, PartialEq)]
pub enum OpEnum {
    Insert(String),
    Update(String, String),
    Delete(String),
}

impl TryFrom<&str> for OpEnum {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s[0..2] {
            "i;" => {
                Ok(Insert(s[2..].to_string()))
            }
            "u;" => {
                let arr: Vec<_> = s[2..].split('|').collect();
                if arr.len() != 2 {
                    return Err(format!("Deserialize update log err: {}", s));
                }
                Ok(Update(arr[0].to_string(), arr[1].to_string()))
            }
            "d;" => {
                Ok(Delete(s[2..].to_string()))
            }
            _ => {
                Err(format!("unknown data: {}", s))
            }
        }
    }
}

impl Display for OpEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Insert(s) => {
                f.write_str(&format!("i;{}", s))
            }
            Update(old, new) => {
                f.write_str(&format!("u;{}|{}", old, new))
            }
            Delete(s) => {
                f.write_str(&format!("d;{}", s))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::sync::models::OpEnum;
    use crate::sync::models::OpEnum::{Delete, Insert, Update};

    #[test]
    fn test_ser() {
        let i = Insert("1".to_string()).to_string();
        assert_eq!(i, "i;1".to_string());

        let i = Update("1".to_string(), "2".to_string()).to_string();
        assert_eq!(i, "u;1|2".to_string());

        let d = Delete("1".to_string()).to_string();
        assert_eq!(d, "d;1".to_string());
    }

    #[test]
    fn test_des() {
        let i: OpEnum = OpEnum::try_from("i;1").unwrap();
        assert_eq!(i, Insert("1".to_string()));

        let u: OpEnum = OpEnum::try_from("u;1|2").unwrap();
        assert_eq!(u, Update("1".to_string(), "2".to_string()));

        let d: OpEnum = OpEnum::try_from("d;1").unwrap();
        assert_eq!(d, Delete("1".to_string()));
    }
}
