use async_trait::async_trait;
use sqlx::pool::PoolConnection;
use sqlx::{Error, Postgres};
use tokio::sync::oneshot::Sender;

type Responder<T> = Sender<Result<T, sqlx::Error>>;

pub type BaseConnection = PoolConnection<Postgres>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum BaseMapperEnum<I, T> {
    ReadDataList { resp: Responder<Vec<T>> },
    ReadDataById { id: I, resp: Responder<T> },
    CreateData { data: T, resp: Responder<T> },
    UpdateData { data: T, resp: Responder<T> },
    DeleteDataById { id: I, resp: Responder<T> },
}

#[macro_export]
macro_rules! resp_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {}
            Err(e) => tracing::error!("Resp failed for {}, error: {:?}", $f, e),
        }
    };
}

#[async_trait]
pub trait BaseMapper<I, T> {
    async fn read_data_list(&self, conn: &mut BaseConnection) -> Result<Vec<T>, Error>;

    async fn read_data_by_id(&self, conn: &mut BaseConnection, id: I) -> Result<T, Error>;

    async fn create_data(&self, conn: &mut BaseConnection, data: T) -> Result<T, Error>;

    async fn update_data(&self, conn: &mut BaseConnection, data: T) -> Result<T, Error>;

    async fn delete_data_by_id(&self, conn: &mut BaseConnection, id: I) -> Result<T, Error>;
}
