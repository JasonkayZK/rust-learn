use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use aquadoggo::{Configuration, NetworkConfiguration, Node};
use gql_client::{Client};
use p2panda_rs::entry::{EncodedEntry, LogId, SeqNum};
use p2panda_rs::entry::decode::decode_entry;
use p2panda_rs::entry::traits::AsEntry;
use p2panda_rs::hash::Hash;
use p2panda_rs::identity::{KeyPair, PublicKey};
use p2panda_rs::operation::EncodedOperation;
use serde::{Deserialize, Serialize};

/// Serializable format holding encoded and signed p2panda operations and entries.
///
/// ```toml
/// version = 1
///
/// [[commits]]
/// entry_hash = "..."
/// entry = "..."
/// operation = "..."
///
/// [[commits]]
/// entry_hash = "..."
/// entry = "..."
/// operation = "..."
///
/// # ...
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockFile {
    pub version: u64,
    pub commits: Option<Vec<Commit>>,
}

/// Single commit with encoded entry and operation pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commit {
    /// Hash of the entry.
    pub entry_hash: Hash,

    /// Encoded and signed p2panda entry.
    pub entry: EncodedEntry,

    /// Encoded p2panda operation.
    pub operation: EncodedOperation,
}

/// GraphQL response for `nextArgs` query.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextArgsResponse {
    pub next_args: NextArguments,
}

/// GraphQL response for `publish` mutation.
// #[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PublishResponse {
    pub publish: NextArguments,
}

/// GraphQL response giving us the next arguments to create an Bamboo entry.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextArguments {
    pub log_id: LogId,
    pub seq_num: SeqNum,
    pub skiplink: Option<Hash>,
    pub backlink: Option<Hash>,
}

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

    let key_pair = get_key_pair(Some(PathBuf::from("schema/secret.txt"))).unwrap();
    println!("Get key pair: private: {:?}, public: {:?}", hex::encode(key_pair.private_key().to_bytes()), hex::encode(key_pair.public_key().to_bytes()));
    let node = Node::start(key_pair, config).await;

    migration().await.expect("migration failed");

    node.on_exit().await;
}

fn get_key_pair(path: Option<PathBuf>) -> Option<KeyPair> {
    let path = path.unwrap_or_else(|| PathBuf::from("secret.txt"));

    // Read private key from file or generate a new one
    let private_key = if Path::exists(&path) {
        let key = read_to_string(path).expect("Couldn't read file!");
        key.replace('\n', "")
    } else {
        return None;
    };

    // Derive key pair from private key
    Some(KeyPair::from_private_key_str(&private_key).expect("Invalid private key"))
}

async fn migration() -> Result<(), String> {
    let mut cli = Client::new("http://localhost:2020/graphql");

    let data = include_str!("../schema/schema.lock");
    let lock_file: LockFile = toml::from_str(data).expect("error parsing schema.lock file");

    // Iterate over all commits which are required to migrate to the latest
    // version. This loop automatically checks if the commit already took place
    // and ignores them if so
    for commit in lock_file.commits.unwrap() {
        // println!("{:?}", commit)

        // Decode entry from commit to retrieve public key, sequence number and log
        // id from it
        let entry = decode_entry(&commit.entry).expect("decode_entry failed");
        let pub_key = entry.public_key();
        let log_id = entry.log_id();
        let seq_num = entry.seq_num();

        // Check if node already knows about this entry
        let next_args_res = next_args(&mut cli, pub_key, commit.entry_hash.to_string()).await;
        if next_args_res.is_some() {
            let next_args_res = next_args_res.unwrap();
            if log_id != &next_args_res.log_id {
                return Err("Critical log id mismatch during migration".parse().unwrap());
            }
            // Entry already exists, we can ignore this commit
            if seq_num < &next_args_res.seq_num {
                continue;
            }
        }

        // Publish commit to node, this will materialize the (updated) schema on
        // the node and give us a new GraphQL API
        let pub_res  = publish(&mut cli, &commit.entry.to_string(), commit.operation.to_string()).await;
        println!("publish res: {:?}", pub_res)
    }

    Ok(())
}

async fn next_args(client: &mut Client, public_key: &PublicKey, view_id: String) -> Option<NextArguments> {
    // 1. Send `nextArgs` GraphQL query to get the arguments from the node to create the next entry
    let query = format!(
        r#"
            {{
                nextArgs(publicKey: "{}", viewId: "{}") {{
                    logId
                    seqNum
                    skiplink
                    backlink
                }}
            }}
            "#,
        public_key,
        view_id,
    );
    let response_result = client.query_unwrap::<NextArgsResponse>(&query).await;
    match response_result {
        Ok(resp) => {
            Some(resp.next_args)
        }
        Err(err) => {
            println!("GraphQL query to fetch `nextArgs` failed: {}", err);
            None
        }
    }
}

async fn publish(client: &mut Client, encoded_entry: &String, encoded_operation: String) -> PublishResponse {
    let query = format!(
        r#"
            mutation Publish {{
                publish(entry: "{}", operation: "{}") {{
                    logId
                    seqNum
                    skiplink
                    backlink
                }}
            }}
        "#,
        encoded_entry, encoded_operation
    );

    client.query_unwrap::<PublishResponse>(&query).await.expect("GraphQL mutation `publish` failed")
}
