use std::collections::HashSet;
use std::sync::OnceLock;

use local_ip_address::local_ip;
use log::{error, info};
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Error};

use crate::storage::StorageHandler;
use crate::storage_proto::storage_client::StorageClient;
use crate::storage_proto::{PingRequest, RegisterRequest};
use crate::utils::{PONG, SYNC_PORT};

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

        tokio::spawn(async move {
            Self::sync_data(&addr).await;
        });
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
        let mut client = Self::get_client(addr).await?;
        let resp = client.ping(PingRequest {}).await?;
        Ok(resp.into_inner().msg.eq(PONG))
    }

    async fn sync_data(addr: &str) {
        let mut client = match Self::get_client(addr).await {
            Ok(c) => c,
            Err(e) => {
                error!("ConnectionRefused: sync data from: {} err: {:#?}", addr, e);
                return;
            }
        };

        let my_local_ip = local_ip().unwrap();
        let data = match client
            .register(RegisterRequest {
                connect_addr: format!("{}:{}", my_local_ip, SYNC_PORT),
            })
            .await
        {
            Ok(resp) => resp.into_inner().data,
            Err(e) => {
                error!("Call register err: {:#?}, local ip: {}", e, my_local_ip);
                return;
            }
        };

        let mut store = StorageHandler::global().lock();
        store.merge_data(&mut data.into_iter().collect());
    }

    async fn get_client(addr: &str) -> Result<StorageClient<Channel>, Error> {
        let addr = Endpoint::from_shared(format!("http://{}", addr))?;
        StorageClient::connect(addr).await
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
