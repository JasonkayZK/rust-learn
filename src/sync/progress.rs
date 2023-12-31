use std::convert::{TryFrom, TryInto};
use std::ops::{BitXor, Range};

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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SyncProgress {
    bitmap: RoaringTreemap,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncEnum {
    Range(Range<u64>),
    Vec(Vec<u64>),
    Single(u64),
}

impl SyncProgress {
    pub fn new() -> Self {
        Self {
            bitmap: RoaringTreemap::new(),
        }
    }

    pub fn get_first_unsynced_index(&self) -> u64 {
        match self.bitmap.max() {
            None => 0,
            Some(right_bound) => {
                // All data is fully synced!
                if self.bitmap.len() == right_bound + 1 {
                    return right_bound + 1;
                }

                let mut mask = RoaringTreemap::new();
                mask.insert_range(0..=right_bound);
                let xor = mask.bitxor(&self.bitmap);
                xor.min().unwrap_or_default()
            }
        }
    }

    /// Indexes should be continuously!
    pub fn get_all_unsynced_sparsed_indexes(&self) -> Vec<u64> {
        match self.bitmap.max() {
            None => vec![],
            Some(right_bound) => {
                let mut mask = RoaringTreemap::new();
                mask.insert_range(0..=right_bound);
                let xor = mask.bitxor(&self.bitmap);
                xor.iter().collect()
            }
        }
    }

    /// Calculate the log indexes that we haven't synced according to `the peer's log length`!
    pub fn calculate_unsynced_indexes(&self, peer_log_length: u64) -> Vec<u64> {
        let max = self.bitmap.max().unwrap_or_default();
        // The data file has been deleted, here we should reset the sync progress
        if peer_log_length < max {
            return (0..peer_log_length).collect();
        }

        let mut mask = RoaringTreemap::new();
        mask.insert_range(0..peer_log_length);
        let xor = mask.bitxor(&self.bitmap);
        xor.iter().collect()
    }

    pub fn set_value(&mut self, v: u64) {
        self.bitmap.insert(v);
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
    fn test_get_first_checkpoint() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        x.set_values(vec![13, 15, 17]);

        assert_eq!(x.get_first_unsynced_index(), 10);
    }

    #[test]
    fn test_get_first_checkpoint_2() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);

        assert_eq!(x.get_first_unsynced_index(), 10);
    }

    #[test]
    fn test_get_all_unsynced_indexes() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        x.set_values(vec![13, 15, 17]);

        assert_eq!(
            x.get_all_unsynced_sparsed_indexes(),
            vec![10, 11, 12, 14, 16]
        );
    }

    #[test]
    fn test_get_all_unsynced_indexes_2() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        assert!(x.get_all_unsynced_sparsed_indexes().is_empty());
    }

    #[test]
    fn test_calculate_unsynced_indexes() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        assert_eq!(x.calculate_unsynced_indexes(15), vec![10, 11, 12, 13, 14]);
    }

    #[test]
    fn test_calculate_unsynced_indexes_2() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        x.set_values(vec![11, 13]);
        assert_eq!(x.calculate_unsynced_indexes(15), vec![10, 12, 14]);
    }

    #[test]
    fn test_calculate_unsynced_indexes_3() {
        let mut x = SyncProgress::new();
        x.set_range(0..10);
        x.set_values(vec![11, 13]);
        assert_eq!(x.calculate_unsynced_indexes(5), vec![0, 1, 2, 3, 4]);
    }
}
