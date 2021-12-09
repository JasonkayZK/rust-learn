use crate::server::router::router;
use crate::CONFIG;
use anyhow::Result;
use hyper::Server as HyperServer;
use routerify::RouterService;
use tokio::sync::mpsc::Sender;
use tracing::info;

pub struct Server<T: Send + Sync + 'static> {
    db_sender: Sender<T>,
}

impl<T: Send + Sync + 'static> Server<T> {
    pub fn new(db_sender: Sender<T>) -> Self {
        Self { db_sender }
    }

    pub async fn listen(&self) -> Result<()> {
        let router = router().data(self.db_sender.clone()).build().unwrap();
        let service = RouterService::new(router).unwrap();

        let address = format!("{}:{}", CONFIG.host, CONFIG.port).parse()?;
        let server = HyperServer::bind(&address).serve(service);
        info!("Server started listening on {}", address);
        server.await?;
        Ok(())
    }
}
