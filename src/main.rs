use crate::configs::conf::CONFIG;
use crate::configs::database::DbHandle;
use crate::dao::base_mapper::BaseMapperEnum;
use crate::dao::url_map_dao::UrlMapDao;
use crate::server::Server;
use anyhow::Result;
use std::sync::Arc;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

mod configs;
mod dao;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    let _ = set_global_default(subscriber);

    let db = DbHandle::new().await.unwrap();
    let db = Arc::new(db);
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = UrlMapDao::new(db, db_rx);
        manager.listen().await;
    });

    Server::new(db_tx).listen().await?;

    Ok(())
}
