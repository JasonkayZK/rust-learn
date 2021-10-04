use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dao::user::{get_user_by_id, User};

#[get("user/{id}")]
async fn get_user(web::Path(id): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(get_user_by_id(id.as_str()))
}

#[get("user/{id}/{name}")]
async fn get_user_by_id_and_name(
    web::Path((id, name)): web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(User { id, name })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user_by_id_and_name)
            .service(get_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
