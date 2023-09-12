use futures::executor::block_on;
use futures::future::{ready, Ready};
use log::debug;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::string::ToString;

use tarpc::context::Context;

use crate::storage::StorageHandler;
use crate::syncer::Syncer;
use crate::utils::PONG;

#[tarpc::service]
pub trait Storage {
    async fn ping() -> String;

    async fn list() -> BTreeSet<String>;

    async fn add(k: String) -> ();

    async fn remove(k: String) -> ();

    async fn register(connect_addr: String) -> BTreeSet<String>;
}

#[derive(Clone)]
pub struct StorageServer;

impl Storage for StorageServer {
    type PingFut = Ready<String>;

    fn ping(self, _ctx: Context) -> Self::PingFut {
        ready(PONG.to_string())
    }

    type ListFut = Ready<BTreeSet<String>>;

    fn list(self, _ctx: Context) -> Self::ListFut {
        let store = StorageHandler::global().lock();
        ready(store.get_copy_data())
    }

    type AddFut = Ready<()>;

    fn add(self, _ctx: Context, k: String) -> Self::AddFut {
        let mut store = StorageHandler::global().lock();
        debug!("add store: {}, success", k);
        store.add(k);
        ready(())
    }

    type RemoveFut = Ready<()>;

    fn remove(self, _ctx: Context, k: String) -> Self::RemoveFut {
        let mut store = StorageHandler::global().lock();
        debug!("remove store: {}, success", k);
        store.remove(&k);
        ready(())
    }

    type RegisterFut = Ready<BTreeSet<String>>;

    fn register(self, _ctx: Context, connect_addr: String) -> Self::RegisterFut {
        block_on(async { Syncer::add_client(connect_addr).await });

        ready(StorageHandler::global().lock().get_copy_data())
    }
}
