use crate::dao::url_map_dao::UrlMap;
use crate::server::State;
use crate::service::url_maps_router;
use crate::{controller, BaseMapperEnum, CONFIG};
use anyhow::{anyhow, Error, Result};
use base64::decode;
use hyper::header::HeaderValue;
use hyper::{Body, Request, Response};
use routerify::{ext::RequestExt, Middleware, RequestInfo, Router, RouterBuilder};
use std::str::from_utf8;
use tracing::{error, info};

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
    let state = req.data::<State<BaseMapperEnum<String, UrlMap>>>().unwrap();
    let sender = state.db_sender();
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
    if req.method() == hyper::Method::OPTIONS {
        return Ok(req);
    }

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

async fn cors(mut res: Response<Body>) -> Result<Response<Body>> {
    let headers = res.headers_mut();
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("*"),
    );
    Ok(res)
}

pub fn router() -> RouterBuilder<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .middleware(Middleware::post(cors))
        .middleware(Middleware::pre(auth_middleware))
        .get("/", home_handler)
        .get("/:key", redirect_handler)
        .scope("/api", url_maps_router())
        .scope("/web", controller::router())
        .err_handler_with_info(error_handler)
}
