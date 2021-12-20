mod url_map_handler;

use anyhow::Error;
use hyper::Body;
use routerify::Router;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/", url_map_handler::index)
        .get("/new", url_map_handler::new)
        .get("/:id/edit", url_map_handler::edit)
        .build()
        .unwrap()
}
