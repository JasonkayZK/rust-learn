use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use chrono::Local;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// There is std::sync::Mutex!
type SharedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
static CAP: usize = 16;

fn new_shared_db(num_shards: usize) -> SharedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = new_shared_db(CAP);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("[{}] Connected to address: {:?}", Local::now(), addr);

        // Clone the handle to the hash map.
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
            println!("[{}] Response to address: {:?}", Local::now(), addr);
        });
    }
}

async fn process(socket: TcpStream, db: SharedDb) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let hashed_key = hash_str(cmd.key());
                println!("Hashed key: {:?}", hashed_key);

                let mut db = db[hashed_key % db.len()].lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let hashed_key = hash_str(cmd.key());
                println!("Hashed key: {:?}", hashed_key);

                let db = db[hashed_key % db.len()].lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

fn hash_str(s: &str) -> usize {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() as usize
}
