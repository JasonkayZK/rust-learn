use crate::api::StorageClient;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::storage::StorageHandler;
use crate::utils::{PONG, SYNC_PORT};
use log::{error, info};
use parking_lot::Mutex;
use tarpc::tokio_serde::formats::Json;
use tarpc::{client, context};

#[derive(Debug, Default)]
pub struct Syncer {
    clients: HashMap<String, StorageClient>,
}

impl Syncer {
    pub async fn add_client(addr: String) {
        if Self::check_client_exist(&addr) {
            return;
        }

        let to_storage_server = tarpc::serde_transport::tcp::connect(&addr, Json::default)
            .await
            .unwrap();
        let client = StorageClient::new(client::Config::default(), to_storage_server).spawn();

        let c = client;
        let e = Self::sync_data(&c).await.err();
        if e.is_some() {
            error!(
                "ConnectionRefused: sync data from: {} err: {}",
                addr,
                e.unwrap()
            );
            return;
        }
        let mut s = Syncer::global().lock();
        s.clients.insert(addr, c.clone());
    }

    pub fn check_client_exist(addr: &str) -> bool {
        let s = Syncer::global().lock();
        s.clients.contains_key(addr)
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
            clients: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    async fn check_health(addr: &str) -> bool {
        let client;
        {
            let s = Syncer::global().lock();
            client = match s.clients.get(addr) {
                None => {
                    return false;
                }
                Some(client) => client,
            }
            .clone();
        }

        match client.ping(context::current()).await {
            Ok(resp) => {
                info!("check health success for: {}", addr);
                resp.eq(PONG)
            }
            Err(e) => {
                error!("check health for {} err: {}", addr, e);
                false
            }
        }
    }

    async fn sync_data(client: &StorageClient) -> anyhow::Result<()> {
        let my_local_ip = local_ip().unwrap();
        let mut data = client
            .register(context::current(), format!("{}:{}", my_local_ip, SYNC_PORT))
            .await?;
        let mut store = StorageHandler::global().lock();
        store.merge_data(&mut data);
        Ok(())
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
