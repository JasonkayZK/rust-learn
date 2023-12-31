use std::convert::{TryFrom, TryInto};
use std::ops::Range;

use redb::{RedbValue, TypeName};
use roaring::RoaringTreemap;
use serde::{Deserialize, Serialize};

/// SyncProgress stores how much progress we've been synced from the other peer
///
/// In another words, SyncProgress shows how much log data we've been received successfully from the other peer servers
///
/// The progress will be increased after received `SyncDataResponse` which means we finally progressed all the corresponding logs!
///
/// SyncLogData -> SyncDataRequest -> SyncDataResponse
#[derive(Debug, PartialEq)]
pub struct SyncProgress {
    bitmap: RoaringTreemap,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncEnum {
    SyncRange(Range<u64>),
    SyncVec(Vec<u64>),
}

impl SyncProgress {
    pub fn new() -> Self {
        Self {
            bitmap: RoaringTreemap::new(),
        }
    }

    pub fn get_first_checkpoint(&self) -> Option<u64> {
        self.bitmap.min()
    }

    pub fn set_range(&mut self, v: Range<u64>) {
        self.bitmap.insert_range(v);
    }

    pub fn set_values(&mut self, v: Vec<u64>) {
        v.iter().for_each(|x| {
            self.bitmap.insert(*x);
        });
    }
}

/// Deserialize for SyncProgress

impl TryFrom<Vec<u8>> for SyncProgress {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Vec<u8>> for SyncProgress {
    type Error = String;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let m = RoaringTreemap::deserialize_from(&mut &value[..]).map_err(|e| e.to_string())?;
        Ok(SyncProgress { bitmap: m })
    }
}

impl TryFrom<&[u8]> for SyncProgress {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let m = RoaringTreemap::deserialize_from(&mut &value[..]).map_err(|e| e.to_string())?;
        Ok(SyncProgress { bitmap: m })
    }
}

/// Serialize for SyncProgress

impl TryFrom<SyncProgress> for Vec<u8> {
    type Error = String;

    fn try_from(value: SyncProgress) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&SyncProgress> for Vec<u8> {
    type Error = String;

    fn try_from(value: &SyncProgress) -> Result<Self, Self::Error> {
        let mut buffer = vec![];
        value
            .bitmap
            .serialize_into(&mut buffer)
            .map_err(|e| e.to_string())?;
        Ok(buffer)
    }
}

impl Default for SyncProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl RedbValue for SyncProgress {
    type SelfType<'a> = Self;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        Self::try_from(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        let v: Vec<u8> = value.try_into().unwrap();
        v
    }

    fn type_name() -> TypeName {
        TypeName::new("SyncProgress")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::sync::progress::SyncProgress;

    #[test]
    fn test_serialize() {
        let x = SyncProgress::new();
        let data: Vec<u8> = (&x).try_into().unwrap();
        assert_eq!(data.len(), x.bitmap.serialized_size());

        let y = SyncProgress::try_from(data).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_set_range() {}
}
