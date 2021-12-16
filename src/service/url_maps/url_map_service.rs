use crate::dao::url_map_dao::UrlMap;
use crate::{json_response, recv_failed_json, sender_failed_json, BaseMapperEnum};
use anyhow::Result;
use hyper::{body::to_bytes, Body, Request, Response};
use routerify::ext::RequestExt;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

pub async fn read_data_list(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    sender_failed_json!(
        sender.send(BaseMapperEnum::ReadDataList { resp: tx }).await,
        "GetUrlMaps"
    );
    let url_maps = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::INTERNAL_SERVER_ERROR);
    Ok(json_response!(body: &url_maps))
}

pub async fn read_data_by_id(req: Request<Body>) -> Result<Response<Body>> {
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    let id = req.param("id").unwrap();
    sender_failed_json!(
        sender
            .send(BaseMapperEnum::ReadDataById {
                id: id.into(),
                resp: tx
            })
            .await,
        "GetUrlMap"
    );
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::NOT_FOUND);
    Ok(json_response!(body: &url_map))
}

pub async fn create_data(mut req: Request<Body>) -> Result<Response<Body>> {
    let body = req.body_mut();
    let url_map_bytes = to_bytes(body).await?;
    let url_map = serde_json::from_slice::<UrlMap>(&url_map_bytes)?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    sender_failed_json!(
        sender
            .send(BaseMapperEnum::CreateData {
                data: url_map,
                resp: tx
            })
            .await,
        "CreateUrlMap"
    );
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::UNPROCESSABLE_ENTITY);
    Ok(json_response!(body: &url_map))
}

pub async fn update_data(mut req: Request<Body>) -> Result<Response<Body>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct UrlMapUrl {
        url: String,
    }

    let body = req.body_mut();
    let url_map_url_bytes = to_bytes(body).await?;
    let url_map_url = serde_json::from_slice::<UrlMapUrl>(&url_map_url_bytes)?;
    let id = req.param("id").unwrap();
    let url_map = UrlMap::new(id.into(), url_map_url.url);
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    sender_failed_json!(
        sender
            .send(BaseMapperEnum::UpdateData {
                data: url_map,
                resp: tx
            })
            .await,
        "UpdateUrlMap"
    );
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::UNPROCESSABLE_ENTITY);
    Ok(json_response!(body: &url_map))
}

pub async fn delete_data_by_id(req: Request<Body>) -> Result<Response<Body>> {
    let id = req.param("id").unwrap();
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed_json!(
        sender
            .send(BaseMapperEnum::DeleteDataById {
                id: id.into(),
                resp: tx
            })
            .await,
        "DeleteUrlMap"
    );
    recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::NOT_FOUND);
    Ok(json_response!(body: &serde_json::json!({
        "ok": "true"
    }).to_string()))
}
