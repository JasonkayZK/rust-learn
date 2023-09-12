use crate::api::StorageClient;
use local_ip_address::local_ip;
use std::collections::{HashSet};
use std::sync::OnceLock;

use crate::storage::StorageHandler;
use crate::utils::{PONG, SYNC_PORT};
use log::{error, info};
use parking_lot::Mutex;
use tarpc::tokio_serde::formats::Json;
use tarpc::{client, context};

#[derive(Debug, Default)]
pub struct Syncer {
    clients: HashSet<String>,
}

impl Syncer {
    pub async fn add_client(addr: String) {
        if Self::check_client_exist(&addr) {
            return;
        }

        let mut s = Syncer::global().lock();
        s.clients.insert(addr.clone());
        let e = Self::sync_data(&addr).await.err();
        if e.is_some() {
            error!(
                "ConnectionRefused: sync data from: {} err: {}",
                addr,
                e.unwrap()
            );
        }
    }

    pub fn check_client_exist(addr: &str) -> bool {
        let s = Syncer::global().lock();
        s.clients.contains(addr)
    }

    fn global() -> &'static Mutex<Syncer> {
        static SYNCER: OnceLock<Mutex<Syncer>> = OnceLock::new();

        SYNCER.get_or_init(|| {
            let d = Mutex::new(Syncer::new());
            info!("init syncer success!");
            d
        })
    }

    fn new() -> Self {
        Self {
            clients: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    async fn check_health(addr: &str) -> anyhow::Result<bool> {
        let client = Self::get_client(addr).await?;
        let resp = client.ping(context::current()).await?;
        Ok(resp.eq(PONG))
    }

    async fn sync_data(addr: &str) -> anyhow::Result<()> {
        let client = Self::get_client(addr).await?;

        let my_local_ip = local_ip().unwrap();
        let mut data = client
            .register(context::current(), format!("{}:{}", my_local_ip, SYNC_PORT))
            .await?;
        let mut store = StorageHandler::global().lock();
        store.merge_data(&mut data);
        Ok(())
    }

    async fn get_client(addr: &str) -> anyhow::Result<StorageClient> {
        let to_storage_server = tarpc::serde_transport::tcp::connect(&addr, Json::default)
            .await
            .unwrap();
        let client = StorageClient::new(client::Config::default(), to_storage_server).spawn();
        Ok(client)
    }
}

#[cfg(test)]
mod test {
    use local_ip_address::local_ip;

    #[test]
    fn test_ip() {
        let my_local_ip = local_ip().unwrap();
        println!("This is my local IP address: {:?}", my_local_ip);
    }
}
