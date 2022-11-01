extern crate chrono;

use std::time::Duration;

use chrono::prelude::*;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("[{}] Connected to address: {:?}", Local::now(), addr);
        tokio::spawn(async move {
            process(socket).await;
            println!("[{}] Response to address: {:?}", Local::now(), addr);
        });
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut conn = Connection::new(socket);

    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        time::sleep(Duration::from_secs(5)).await;

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}
