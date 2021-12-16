use crate::dao::url_map_dao::UrlMap;
use crate::service::url_maps_router;
use crate::{BaseMapperEnum, CONFIG};
use anyhow::{anyhow, Error, Result};
use base64::decode;
use hyper::{Body, Request, Response};
use routerify::{ext::RequestExt, Middleware, RequestInfo, Router, RouterBuilder};
use std::str::from_utf8;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};

macro_rules! sender_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {}
            Err(e) => {
                error!("Database Manager failed to get {}! error: {}", $f, e);
                return Ok(Response::builder()
                    .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Something went wrong: {}", e)))
                    .unwrap());
            }
        }
    };
}

macro_rules! recv_failed {
    ($m: expr) => {
        match $m {
            Ok(d) => d,
            Err(e) => {
                error!("Database Manager returned error: {}", e);
                return Ok(Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .body(Body::from("Key does not exist"))
                    .unwrap());
            }
        }
    };
}

async fn logger(req: Request<Body>) -> Result<Request<Body>> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    let status = match err.to_string().as_str() {
        "Unauthorized Access" => hyper::StatusCode::UNAUTHORIZED,
        _ => hyper::StatusCode::INTERNAL_SERVER_ERROR,
    };
    Response::builder()
        .status(status)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

async fn home_handler(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Url Mapper in Rust!")))
}

async fn redirect_handler(req: Request<Body>) -> Result<Response<Body>> {
    let sender = req
        .data::<Sender<BaseMapperEnum<String, UrlMap>>>()
        .unwrap();
    let key = req.param("key").unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed!(
        sender
            .send(BaseMapperEnum::ReadDataById {
                id: key.clone(),
                resp: tx,
            })
            .await,
        "GetUrlMap"
    );
    let url_map = recv_failed!(rx.await.unwrap());
    Ok(Response::builder()
        .header(hyper::header::LOCATION, url_map.url.clone())
        .status(hyper::StatusCode::SEE_OTHER)
        .body(Body::from(format!("redirecting to url: {}", url_map.url)))
        .unwrap())
}

async fn auth_middleware(req: Request<Body>) -> Result<Request<Body>> {
    let auth_token_header = req.headers().get(hyper::header::AUTHORIZATION);
    match auth_token_header {
        None => Err(anyhow!("Unauthorized Access")),
        Some(auth_token) => {
            let token = auth_token.to_str()?;
            validate_token(token)?;
            Ok(req)
        }
    }
}

fn validate_token(encoded_token: &str) -> Result<()> {
    let auth_token_bytes = decode(&encoded_token)?;
    let auth_token = from_utf8(&auth_token_bytes)?;
    if auth_token != CONFIG.auth_token.as_str() {
        return Err(anyhow!("Unauthorized Access"));
    }
    Ok(())
}

pub fn router() -> RouterBuilder<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .middleware(Middleware::pre(auth_middleware))
        .get("/", home_handler)
        .get("/:key", redirect_handler)
        .scope("/api", url_maps_router())
        .err_handler_with_info(error_handler)
}
