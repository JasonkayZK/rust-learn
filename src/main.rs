use crate::configs::conf::CONFIG;
use crate::configs::database::DbHandle;
use crate::dao::base_mapper::BaseMapperEnum;
use crate::dao::url_map_dao::{UrlMap, UrlMapDao};
use anyhow::Result;
use std::sync::Arc;

mod configs;
mod dao;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{:?}", CONFIG);

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

    // Create
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx
        .send(BaseMapperEnum::CreateData {
            data: UrlMap {
                key: String::from("linkedin"),
                url: String::from("linkedin.com"),
            },
            resp: tx,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    // Read
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx
        .send(BaseMapperEnum::ReadDataById {
            id: "linkedin".into(),
            resp: tx,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    // Update
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx
        .send(BaseMapperEnum::UpdateData {
            data: UrlMap {
                key: String::from("linkedin"),
                url: String::from("linkedin.com2"),
            },
            resp: tx,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    // Read Again
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx
        .send(BaseMapperEnum::ReadDataById {
            id: "linkedin".into(),
            resp: tx,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    // Delete
    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx
        .send(BaseMapperEnum::DeleteDataById {
            id: "linkedin".into(),
            resp: tx,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to send to database manager: {}", e),
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e),
    }

    Ok(())
}
