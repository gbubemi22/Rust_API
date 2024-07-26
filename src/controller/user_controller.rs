use crate::{service::user_service::UserService, utils::model::LoginRequests};
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;


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
    match user_service
        .create_user(
            user_info.username.clone(),
            user_info.email.clone(),
            user_info.password.clone(),
        )
        .await
    {
        Ok(user_id) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User created successfully",
            "user_id": user_id.to_hex()
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Failed to create user",
            "error": e.to_string()
        })),
    }
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}
#[allow(dead_code)]
#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login_user(
    user_service: web::Data<UserService>,
    login_info: web::Json<LoginRequests>,
) -> impl Responder {
    match user_service
        .login_fn(login_info.into_inner())
        .await
    {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "token": token
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}
