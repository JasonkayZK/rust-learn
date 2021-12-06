use crate::configs::conf::CONFIG;
use crate::configs::database::DbHandle;
use crate::dao::base_mapper::BaseMapperEnum;
use crate::dao::url_map_dao::UrlMapDao;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

mod configs;
mod dao;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    let _ = set_global_default(subscriber);

    info! {
        "{:?}", CONFIG
    }

    let db = DbHandle::new().await.unwrap();
    let db = Arc::new(db);
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = UrlMapDao::new(db, db_rx);
        manager.listen().await;
    });

    // ReadList
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx.send(BaseMapperEnum::ReadDataList { resp: tx }).await {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    server::listen().await?;

    Ok(())
}
