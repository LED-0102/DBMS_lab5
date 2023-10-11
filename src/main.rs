mod auth;
use actix_web::{web, HttpResponse, Responder, HttpServer, App};
use auth::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}