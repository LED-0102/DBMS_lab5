mod auth;
use actix_web::{web, HttpResponse, Responder, HttpServer, App};
use auth::config;
use auth::jwt::JwToken;
use actix_cors::Cors;
use actix_web::dev::Service;

async fn welcome (token: JwToken) -> HttpResponse {
    HttpResponse::Ok().body("Welcome to IIT Indore!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .wrap_fn(|req, srv| {
                println!("Hi boi got it {} {}", req.method(), req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(config)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
            .route("/welcome", web::post().to(welcome))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}