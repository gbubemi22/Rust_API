use actix_web::web;
use crate::controller::user_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(
                web::scope("/users")
                    .route("/register", web::post().to(user_controller::register_user))
                    // Add more routes here
            )
    );
}