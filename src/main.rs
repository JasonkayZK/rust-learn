use crate::configs::conf::CONFIG;
use crate::configs::database::DbHandle;
use crate::dao::url_map_dao::UrlMap;
use anyhow::Result;

mod configs;
mod dao;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{:?}", CONFIG);

    let db = DbHandle::new().await.unwrap();
    let res = sqlx::query_as::<_, UrlMap>("select * from url_maps")
        .fetch_all(&db.pool)
        .await?;
    println!("result: {:?}", res);

    Ok(())
}
