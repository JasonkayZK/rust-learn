use crate::configs::conf::CONFIG;
use crate::configs::database::DbHandle;
use crate::dao::base_mapper::BaseMapperEnum;
use crate::dao::url_map_dao::UrlMapDao;
use crate::server::Server;
use anyhow::Result;
use std::process;
use std::sync::Arc;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

mod configs;
mod controller;
mod dao;
#[macro_use]
mod macros;
mod server;
mod service;

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

    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};
        let mut hup = signal(SignalKind::hangup()).unwrap();
        let mut int = signal(SignalKind::interrupt()).unwrap();
        let mut quit = signal(SignalKind::quit()).unwrap();
        let mut term = signal(SignalKind::terminate()).unwrap();

        tokio::select! {
            _ = hup.recv() => tracing::info!("Recieved SIGHUP!"),
            _ = int.recv() => tracing::info!("Recieved SIGINT!"),
            _ = quit.recv() => tracing::info!("Recieved SIGQUIT!"),
            _ = term.recv() => tracing::info!("Recieved SIGTERM!"),
        }
        tracing::info!("Good Bye from Url Mapper in Rust!");
        process::exit(0);
    });

    Server::new(db_tx).listen().await?;

    Ok(())
}
