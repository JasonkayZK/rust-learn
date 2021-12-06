use crate::server::router::router;
use crate::CONFIG;
use anyhow::Result;
use hyper::Server;
use routerify::RouterService;
use tracing::info;

pub async fn listen() -> Result<()> {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let address = format!("{}:{}", CONFIG.host, CONFIG.port).parse()?;

    let server = Server::bind(&address).serve(service);
    info!("Server started listening on {}", address);
    server.await?;
    Ok(())
}
