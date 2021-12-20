use anyhow::Error;
use hyper::Body;
use routerify::Router;

pub mod url_maps;

pub fn url_maps_router() -> Router<Body, Error> {
    Router::builder()
        .scope("/url_maps", url_maps::router())
        .build()
        .unwrap()
}
