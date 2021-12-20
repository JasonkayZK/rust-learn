use crate::{CONFIG};
use anyhow::Result;
use tera::Tera;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct State<T> {
    dao: Sender<T>,
    tera: Tera,
}

impl<T: Send + Sync + 'static> State<T> {
    pub fn new(dao: Sender<T>) -> Result<Self> {
        let tera = Tera::new("client/tera/**/*.html")?;
        Ok(Self { dao, tera })
    }

    pub fn db_sender(&self) -> Sender<T> {
        self.dao.clone()
    }

    pub fn tera(&self) -> Tera {
        let mut tera = self.tera.clone();
        if CONFIG.env.as_str() == "development" {
            match tera.full_reload() {
                Ok(_) => tracing::info!("Tera templates reloaded successfully!"),
                Err(e) => tracing::error!("Failed to reload tera templates: {}", e),
            }
        }
        tera
    }
}
