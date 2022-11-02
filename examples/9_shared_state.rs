use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use chrono::Local;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// There is std::sync::Mutex!
type ArcM<T> = Arc<Mutex<T>>;

type Db = ArcM<HashMap<String, Bytes>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

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

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
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
