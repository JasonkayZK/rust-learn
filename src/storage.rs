use std::collections::{BTreeSet};
use std::sync::{OnceLock};

use boost_rs::rand::string::get_random_alphanumeric_string;
use log::info;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct StorageHandler {
    data: BTreeSet<String>,
}

impl StorageHandler {
    // init global
    pub fn global() -> &'static Mutex<StorageHandler> {
        static STORAGE: OnceLock<Mutex<StorageHandler>> = OnceLock::new();

        STORAGE.get_or_init(|| Mutex::new(StorageHandler::new()))
    }

    fn new() -> Self {
        let mut d = BTreeSet::from([
            "1".to_string(),
        ]);

        for _ in 0..2 {
            d.insert(get_random_alphanumeric_string(5));
        }

        info!("load data success: {:#?}", d);

        Self {
            data: d,
        }
    }

    pub fn add(&mut self, k: String) {
        self.data.insert(k);
    }

    pub fn remove(&mut self, k: &str) -> bool {
        self.data.remove(k)
    }

    pub fn get_copy_list(&self) -> Vec<String> {
        self.data.iter().map(|x| x.to_string()).collect()
    }

    pub fn print(&self) {
        info!("{:#?}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::StorageHandler;

    #[test]
    fn test_main() {
        let s = StorageHandler::global();
        s.lock().print();
        assert_eq!(s.lock().data.len(), 10);

        s.lock().add("4".to_string());
        s.lock().print();
        assert_eq!(s.lock().data.len(), 11);

        s.lock().remove("1");
        s.lock().print();
        assert_eq!(s.lock().data.len(), 10);

        s.lock().remove("5");
        s.lock().print();
        assert_eq!(s.lock().data.len(), 10);
    }
}
