use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uhlc::Timestamp;

use crate::sync::models::OpEnum::{Delete, Insert, Update};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OpEnum {
    Insert(u64, Timestamp),
    Update(u64, u64, Timestamp),
    Delete(u64, Timestamp),
}

impl TryFrom<Vec<u8>> for OpEnum {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(String::from_utf8(value).unwrap().as_str())
    }
}

impl TryFrom<&str> for OpEnum {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s[0..2] {
            "i;" => {
                let arr: Vec<_> = s[2..].split(';').collect();
                if arr.len() != 2 {
                    return Err(format!("Deserialize update log err: {}", s));
                }
                let data_hash = arr[0].parse::<u64>().unwrap();
                let timestamp = Timestamp::from_str(arr[1]).map_err(|e| e.cause)?;
                Ok(Insert(data_hash, timestamp))
            }
            "u;" => {
                let arr: Vec<_> = s[2..].split(';').collect();
                if arr.len() != 2 {
                    return Err(format!("Deserialize update log err: {}", s));
                }
                let timestamp = Timestamp::from_str(arr[1]).map_err(|e| e.cause)?;
                let data_hash_arr: Vec<_> = arr[0].split('|').collect();
                if data_hash_arr.len() != 2 {
                    return Err(format!("Deserialize update log err: {}", s));
                }
                Ok(Update(
                    data_hash_arr[0].parse::<u64>().unwrap(),
                    data_hash_arr[1].parse::<u64>().unwrap(),
                    timestamp,
                ))
            }
            "d;" => {
                let arr: Vec<_> = s[2..].split(';').collect();
                if arr.len() != 2 {
                    return Err(format!("Deserialize update log err: {}", s));
                }
                let data_hash = arr[0].parse::<u64>().unwrap();
                let timestamp = Timestamp::from_str(arr[1]).map_err(|e| e.cause)?;
                Ok(Delete(data_hash, timestamp))
            }
            _ => Err(format!("unknown data: {}", s)),
        }
    }
}

impl Display for OpEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Insert(s, t) => f.write_str(&format!("i;{};{}", s, t)),
            Update(old, new, t) => f.write_str(&format!("u;{}|{};{}", old, new, t)),
            Delete(s, t) => f.write_str(&format!("d;{};{}", s, t)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    use uhlc::Timestamp;

    use crate::sync::models::OpEnum;
    use crate::sync::models::OpEnum::{Delete, Insert, Update};

    #[test]
    fn test_ser() {
        let i = Insert(
            1,
            Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
        )
        .to_string();
        assert_eq!(i, "i;1;1970-01-01T00:00:00.000000000Z/1".to_string());

        let i = Update(
            1,
            2,
            Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
        )
        .to_string();
        assert_eq!(i, "u;1|2;1970-01-01T00:00:00.000000000Z/1".to_string());

        let d = Delete(
            1,
            Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
        )
        .to_string();
        assert_eq!(d, "d;1;1970-01-01T00:00:00.000000000Z/1".to_string());
    }

    #[test]
    fn test_des() {
        let i: OpEnum = OpEnum::try_from("i;1;1970-01-01T00:00:00.000000000Z/1").unwrap();
        assert_eq!(
            i,
            Insert(
                1,
                Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
            )
        );

        let u: OpEnum = OpEnum::try_from("u;1|2;1970-01-01T00:00:00.000000000Z/1").unwrap();
        assert_eq!(
            u,
            Update(
                1,
                2,
                Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
            )
        );

        let d: OpEnum = OpEnum::try_from("d;1;1970-01-01T00:00:00.000000000Z/1").unwrap();
        assert_eq!(
            d,
            Delete(
                1,
                Timestamp::from_str("1970-01-01T00:00:00.000000000Z/1").unwrap(),
            )
        );
    }
}
