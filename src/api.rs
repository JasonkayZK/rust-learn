use std::fmt::Debug;
use futures::future::{Ready, ready};
use log::debug;
use tarpc::context::Context;

use crate::storage::StorageHandler;

#[tarpc::service]
pub trait Storage {
    async fn list() -> Vec<String>;

    async fn add(k: String) -> ();

    async fn remove(k: String) -> ();

    async fn sync(connect_addr: String) -> ();
}

#[derive(Clone)]
pub struct StorageServer;

impl Storage for StorageServer {
    type ListFut = Ready<Vec<String>>;

    fn list(self, _ctx: Context) -> Self::ListFut {
        let store = StorageHandler::global().lock();
        ready(store.get_copy_list())
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

    type SyncFut = Ready<()>;

    fn sync(self, _ctx: Context, connect_addr: String) -> Self::SyncFut {

        // tarpc::serde_transport::tcp::connect(connect_addr, Json::default)
        // let storage_client = StorageClient::new(client::Config::default(), to_storage_server).spawn();
        //
        // let ctx = context::current();
        // storage_client.add(ctx, boost_rs::rand::string::get_random_alphanumeric_string(3)).await.unwrap();
        // println!("list: {:#?}", storage_client.list(ctx).await.unwrap())
        todo!()
    }
}
