use crate::dao::base_mapper::{BaseConnection, BaseMapper, BaseMapperEnum};
use crate::{resp_failed, DbHandle};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UrlMap {
    pub key: String,
    pub url: String,
}

pub struct UrlMapDao {
    db: Arc<DbHandle>,
    receiver: Receiver<BaseMapperEnum<String, UrlMap>>,
}

#[async_trait]
impl BaseMapper<String, UrlMap> for UrlMapDao {
    async fn read_data_list(&self, conn: &mut BaseConnection) -> Result<Vec<UrlMap>, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("SELECT * FROM url_maps")
            .fetch_all(conn)
            .await
    }

    async fn read_data_by_id(
        &self,
        conn: &mut BaseConnection,
        id: String,
    ) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("SELECT * FROM url_maps WHERE key = $1")
            .bind(id)
            .fetch_one(conn)
            .await
    }

    async fn create_data(
        &self,
        conn: &mut BaseConnection,
        data: UrlMap,
    ) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("INSERT INTO url_maps (key, url) VALUES ($1, $2) RETURNING *")
            .bind(data.key)
            .bind(data.url)
            .fetch_one(conn)
            .await
    }

    async fn update_data(
        &self,
        conn: &mut BaseConnection,
        data: UrlMap,
    ) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("UPDATE url_maps SET url=$1 WHERE key=$2 RETURNING *")
            .bind(data.url)
            .bind(data.key)
            .fetch_one(conn)
            .await
    }

    async fn delete_data_by_id(
        &self,
        conn: &mut BaseConnection,
        id: String,
    ) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("DELETE FROM url_maps WHERE key = $1 RETURNING *")
            .bind(id)
            .fetch_one(conn)
            .await
    }
}

impl UrlMapDao {
    pub fn new(db: Arc<DbHandle>, receiver: Receiver<BaseMapperEnum<String, UrlMap>>) -> Self {
        Self {
            db: Arc::clone(&db),
            receiver,
        }
    }

    pub async fn listen(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            let mut connection = self.db.pool.acquire().await.unwrap();
            match message {
                BaseMapperEnum::ReadDataList { resp } => {
                    let url_maps = Self::read_data_list(self, &mut connection).await;
                    resp_failed!(resp.send(url_maps), "GetUrlMaps");
                }
                BaseMapperEnum::ReadDataById { id, resp } => {
                    let url_map = Self::read_data_by_id(self, &mut connection, id).await;
                    resp_failed!(resp.send(url_map), "GetUrlMap");
                }
                BaseMapperEnum::CreateData { data, resp } => {
                    let url_map = Self::create_data(self, &mut connection, data).await;
                    resp_failed!(resp.send(url_map), "CreateUrlMap");
                }
                BaseMapperEnum::UpdateData { data, resp } => {
                    let url_map = Self::update_data(self, &mut connection, data).await;
                    resp_failed!(resp.send(url_map), "UpdateUrlMap");
                }
                BaseMapperEnum::DeleteDataById { id, resp } => {
                    let url_map = Self::delete_data_by_id(self, &mut connection, id).await;
                    resp_failed!(resp.send(url_map), "DeleteUrlMap");
                }
            }
        }
    }
}
