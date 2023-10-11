use actix_web::web;

pub mod login;
pub mod logout;
pub mod register;
mod jwt;
use login::login;
use logout::logout;
use register::register;

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
    );
}