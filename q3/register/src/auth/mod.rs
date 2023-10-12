use actix_web::web;

pub mod register;
use register::register;

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
    );
}