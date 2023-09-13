use std::collections::HashSet;
use std::sync::OnceLock;

use anyhow::Result;
use local_ip_address::local_ip;
use log::{debug, error, info};
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Error};

use crate::storage::StorageHandler;
use crate::storage_proto::storage_client::StorageClient;
use crate::storage_proto::{AddRequest, PingRequest, RegisterRequest, RemoveRequest};
use crate::utils::{PONG, SYNC_PORT};

#[derive(Debug, Default)]
pub struct Syncer {
    clients: HashSet<String>,
}

#[derive(Debug)]
pub enum SyncOptEnum {
    Add,
    Remove,
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

    pub fn sync_opt(opt: SyncOptEnum, data: String) {
        let client_list;
        {
            client_list = Self::global().lock().clients.clone();
        }
        let mut disconnected_addr = vec![];

        debug!(
            "Start sync opt: {:?}, current client list: {:#?}",
            opt, client_list
        );

        tokio::spawn(async move {
            for client_addr in client_list.iter() {
                // Step 1: Get client
                let mut rpc_cli = match Self::get_client(client_addr).await {
                    Ok(cli) => cli,
                    Err(e) => {
                        error!("Get client for address: {} err: {}", client_addr, e);
                        disconnected_addr.push(client_addr.clone());
                        continue;
                    }
                };
                // Step 2: Check health
                match Self::check_health(&mut rpc_cli).await {
                    Ok(is_health) => {
                        if !is_health {
                            error!("Checked unhealthy for address: {}", client_addr);
                            disconnected_addr.push(client_addr.clone());
                            continue;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Check health failed for address: {} err: {}",
                            client_addr, e
                        );
                        disconnected_addr.push(client_addr.clone());
                        continue;
                    }
                }
                // Step 3: Opt
                match opt {
                    SyncOptEnum::Add => {
                        Self::sync_add(&mut rpc_cli, data.clone(), client_addr).await;
                    }
                    SyncOptEnum::Remove => {
                        Self::sync_remove(&mut rpc_cli, data.clone(), client_addr).await;
                    }
                };

                debug!("Sync Opt: {:?} success, data: {}", opt, data);
                debug!(
                    "Current data: {:#?}",
                    StorageHandler::global().lock().get_copy_data()
                )
            }
        });
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

    async fn check_health(client: &mut StorageClient<Channel>) -> Result<bool> {
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

    async fn sync_add(client: &mut StorageClient<Channel>, data: String, addr: &str) {
        match client.add(AddRequest { key: data.clone() }).await {
            Err(e) => {
                error!("Sync[Add] data: {} for addr: {} error: {}", data, addr, e);
            }
            _ => {
                debug!("Sync[Add] data: {} for addr: {} success", data, addr)
            }
        };
    }

    async fn sync_remove(client: &mut StorageClient<Channel>, data: String, addr: &str) {
        match client.remove(RemoveRequest { key: data.clone() }).await {
            Err(e) => {
                error!(
                    "Sync[Remove] data: {} for addr: {} error: {}",
                    data, addr, e
                );
            }
            _ => {
                debug!("Sync[Remove] data: {} for addr: {} success", data, addr)
            }
        }
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
