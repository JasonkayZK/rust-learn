use anyhow::Error;
use hyper::Body;
use routerify::Router;

mod url_map_service;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/:id", url_map_service::read_data_by_id)
        .get("/", url_map_service::read_data_list)
        .post("/", url_map_service::create_data)
        .put("/:id", url_map_service::update_data)
        .delete("/:id", url_map_service::delete_data_by_id)
        .build()
        .unwrap()
}
