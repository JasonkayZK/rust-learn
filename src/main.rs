use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use aquadoggo::{Configuration, NetworkConfiguration, Node};
use graphql_client::{GraphQLQuery, Response};
use p2panda_rs::entry::decode::decode_entry;
use p2panda_rs::entry::traits::AsEntry;
use p2panda_rs::entry::EncodedEntry;
use p2panda_rs::hash::Hash;
use p2panda_rs::identity::{KeyPair, PublicKey};
use p2panda_rs::operation::EncodedOperation;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::graphql::next_args::NextArgsNextArgs;
use crate::graphql::publish::PublishPublish;
use crate::graphql::{next_args, publish, NextArgs, Publish};

mod graphql;

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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let http_port = args.get(1).map_or(2020u16, |x| x.parse().unwrap());
    let node_port = args.get(2).map_or(2022u16, |x| x.parse().unwrap());

    let config = Configuration {
        database_url: "sqlite://test.sqlite".into(),
        http_port,
        network: NetworkConfiguration {
            quic_port: node_port,
            ..Default::default()
        },
        ..Default::default()
    };

    let key_pair = KeyPair::new();
    let node = Node::start(key_pair, config).await;

    migration().await.expect("migration failed");

    node.on_exit().await;
}

pub fn get_key_pair(path: Option<PathBuf>) -> Option<KeyPair> {
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

async fn migration() -> Result<()> {
    let mut cli = Client::new();

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
        let next_args_res = next_args(&mut cli, *pub_key, commit.entry_hash.to_string()).await?;
        if next_args_res.is_some() {
            let next_args_res = next_args_res.unwrap();
            if log_id != &next_args_res.log_id {
                bail!("Critical log id mismatch during migration");
            }
            // Entry already exists, we can ignore this commit
            if seq_num < &next_args_res.seq_num {
                continue;
            }
        }

        // Publish commit to node, this will materialize the (updated) schema on
        // the node and give us a new GraphQL API
        let pub_res = publish(&mut cli, commit.entry, commit.operation).await;
        println!("publish res: {:?}", pub_res)
    }

    Ok(())
}

async fn next_args(
    client: &mut Client,
    public_key: PublicKey,
    view_id: String,
) -> Result<Option<NextArgsNextArgs>> {
    let request_body = NextArgs::build_query(next_args::Variables {
        pk: public_key,
        vid: Some(view_id),
    });

    let res = client
        .post("http://localhost:2020/graphql")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<next_args::ResponseData> = res.json().await?;

    println!("next_args response: {:?}", response_body);

    match response_body.data {
        None => Ok(None),
        Some(res) => Ok(res.next_args),
    }
}

async fn publish(
    client: &mut Client,
    encoded_entry: EncodedEntry,
    encoded_operation: EncodedOperation,
) -> Result<Option<PublishPublish>> {
    let request_body = Publish::build_query(publish::Variables {
        entry: encoded_entry,
        operation: encoded_operation,
    });
    let res = client
        .post("http://localhost:2020/graphql")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<publish::ResponseData> = res.json().await?;

    match response_body.data {
        None => Ok(None),
        Some(res) => Ok(Some(res.publish)),
    }
}

#[cfg(test)]
mod tests {
    use graphql_client::{GraphQLQuery, Response};

    use crate::graphql::{all_schema_field_definitions, AllSchemaFieldDefinitions};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let request_body =
            AllSchemaFieldDefinitions::build_query(all_schema_field_definitions::Variables {});
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:2020/graphql")
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<all_schema_field_definitions::ResponseData> =
            res.json().await?;
        println!(
            "{:#?}",
            response_body
                .data
                .unwrap()
                .all_schema_field_definition_v1
                .documents
        );
        Ok(())
    }
}
