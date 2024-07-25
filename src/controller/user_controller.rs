use actix_web::{web, HttpResponse, Responder};
use crate::service::user_service::UserService;

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn register_user(
    user_service: web::Data<UserService>,
    user_info: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service.create_user(user_info.username.clone(), user_info.email.clone(), user_info.password.clone()).await {
        Ok(user_id) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User created successfully",
            "user_id": user_id.to_hex()
        })),
        Err(e) => {
            match e.as_str() {
                "Email already exists" | "Username already exists" => HttpResponse::Conflict().json(serde_json::json!({
                    "message": e
                })),
                _ => HttpResponse::InternalServerError().json(serde_json::json!({
                    "message": "Failed to create user",
                    "error": e
                })),
            }
        }
    }
}