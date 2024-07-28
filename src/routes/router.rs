use crate::controller::{todo_controller, user_controller};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(
                web::scope("/users")
                    .route("/register", web::post().to(user_controller::register_user))
                    .route("/login", web::post().to(user_controller::login_user)), // Add more routes here
            )
            .service(
                web::scope("/todos").route("", web::post().to(todo_controller::create_todo)),
                // Add more routes here
                // .route("", web::get().to(todo_controller::list_todos))
                // .route("/{id}", web::get().to(todo_controller::get_todo))
                // .route("/{id}", web::put().to(todo_controller::update_todo))
                // .route("/{id}", web::delete().to(todo_controller::delete_todo)),
            ),
    );
}
