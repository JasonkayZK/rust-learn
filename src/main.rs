use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use aquadoggo::{Configuration, NetworkConfiguration, Node};
use p2panda_rs::identity::KeyPair;

const KEY_PAIR_FILE: &str = "key_pair.json";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let http_port = args.get(1).map_or(2020u16, |x| { x.parse().unwrap() });
    let node_port = args.get(2).map_or(2022u16, |x| { x.parse().unwrap() });

    let config = Configuration {
        database_url: "sqlite://test.sqlite".into(),
        http_port,
        network: NetworkConfiguration {
            quic_port: node_port,
            ..Default::default()
        },
        ..Default::default()
    };

    let key_pair = get_key_pair();
    save_pk_for_client(hex::encode(key_pair.private_key().to_bytes()));
    let node = Node::start(key_pair, config).await;

    node.on_exit().await;
}

fn get_key_pair() -> KeyPair {
    if Path::new(KEY_PAIR_FILE).exists() { // Key already generated
        let mut key_pair_file = File::open(KEY_PAIR_FILE).unwrap();
        let mut contents = String::new();
        key_pair_file.read_to_string(&mut contents).unwrap();
        let kp: KeyPair = serde_json::from_str(&contents).unwrap();
        println!("Load key pair successfully!");
        return kp;
    }
    // Generate new key pair and save
    let kp = KeyPair::new();
    let serialized = serde_json::to_string(&kp).unwrap();
    let mut file = File::create(KEY_PAIR_FILE).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("New key pair generated and saved successfully");
    kp
}

fn save_pk_for_client(pk: String) {
    let mut file = File::create("key.txt").unwrap();
    file.write_all(pk.as_ref()).unwrap();
}
