use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use log::debug;

use crate::{middleware::auth::verify_token, service::todo_service::TodoService};
#[derive(serde::Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

pub async fn create_todo(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    todo_info: web::Json<CreateTodoRequest>,
) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        debug!("Authorization header: {:?}", auth_header);
    } else {
        debug!("No Authorization header found");
    }
    // Extract user_id from token
    let user_id = match verify_token(&req).await {
        Ok(id) => match ObjectId::parse_str(&id) {
            Ok(object_id) => object_id,
            Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Invalid user ID format in token"
            })),
        },
        Err(_) => return HttpResponse::Unauthorized().json(serde_json::json!({
            "message": "Invalid or missing token"
        })),
    };
    println!("{}", user_id);

    // Create todo
    match todo_service
        .create_todo(
            todo_info.title.clone(),
            todo_info.description.clone(),
            user_id,
        )
        .await
    {
        Ok(todo_id) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Todo created successfully",
            "todo_id": todo_id.to_hex()
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Failed to create todo",
            "error": e.to_string()
        })),
    }
}