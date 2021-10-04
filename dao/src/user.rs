use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref USER_MAP: HashMap<&'static str, User> = {
        let mut m = HashMap::new();
        m.insert(
            "0",
            User {
                id: "0".to_string(),
                name: "foo".to_string(),
            },
        );
        m.insert(
            "1",
            User {
                id: "1".to_string(),
                name: "bar".to_string(),
            },
        );
        m.insert(
            "2",
            User {
                id: "2".to_string(),
                name: "baz".to_string(),
            },
        );

        m
    };
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

pub fn get_user_by_id(id: &str) -> Option<&User> {
    USER_MAP.get(&id)
}
