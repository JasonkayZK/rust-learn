mod url_maps_controller;

use crate::CONFIG;
use anyhow::{Error, Result};
use hyper::{Body, Request, Response};
use routerify::Router;
use std::fs::read_to_string;

async fn css_handler(_req: Request<Body>) -> Result<Response<Body>> {
    let css = match CONFIG.env.as_str() {
        "development" => read_to_string("client/tera/style.css")?,
        _ => include_str!("../../client/tera/style.css").to_string(),
    };
    Ok(Response::builder().body(Body::from(css)).unwrap())
}

async fn js_handler(_req: Request<Body>) -> Result<Response<Body>> {
    let js = match CONFIG.env.as_str() {
        "development" => read_to_string("client/tera/index.js")?,
        _ => include_str!("../../client/tera/index.js").to_string(),
    };
    Ok(Response::builder().body(Body::from(js)).unwrap())
}

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/index.js", js_handler)
        .get("/style.css", css_handler)
        .scope("/url_maps", url_maps_controller::router())
        .build()
        .unwrap()
}
