use crate::{model::todo_model::Todo, utils::error::CustomError};
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use log::debug;
use mongodb::bson::oid::ObjectId;

use crate::{middleware::auth::verify_token, service::todo_service::TodoService};
#[derive(serde::Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateTodoRequest {
    title: Option<String>,       // Optional field for title
    description: Option<String>, // Optional field for description
    completed: Option<bool>,     // Optional field for completion status
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
            Err(_) => {
                return CustomError::BadRequestError("Invalid user ID format in token".to_string())
                    .error_response()
            }
        },
        Err(_) => {
            return CustomError::UnauthorizedError("Invalid or missing token".to_string())
                .error_response()
        }
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

pub async fn list_all(req: HttpRequest, todo_service: web::Data<TodoService>) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        debug!("Authorization header: {:?}", auth_header);
    } else {
        debug!("No Authorization header found");
    }
    // Extract user_id from token
    let user_id = match verify_token(&req).await {
        Ok(id) => match ObjectId::parse_str(&id) {
            Ok(object_id) => object_id,
            Err(_) => {
                return CustomError::BadRequestError("Invalid user ID format in token".to_string())
                    .error_response()
            }
        },
        Err(_) => {
            return CustomError::UnauthorizedError("Invalid or missing token".to_string())
                .error_response()
        }
    };
    println!("{}", user_id);

    // Fetch todos for the user
    match todo_service.list_todos(user_id).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn list_one(req: HttpRequest, todo_service: web::Data<TodoService>) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        debug!("Authorization header: {:?}", auth_header);
    } else {
        debug!("No Authorization header found");
    }
    // Extract user_id from token
    let user_id = match verify_token(&req).await {
        Ok(id) => match ObjectId::parse_str(&id) {
            Ok(object_id) => object_id,
            Err(_) => {
                return CustomError::BadRequestError("Invalid user ID format in token".to_string())
                    .error_response()
            }
        },
        Err(_) => {
            return CustomError::UnauthorizedError("Invalid or missing token".to_string())
                .error_response()
        }
    };
    println!("{}", user_id);

    let id = match req.match_info().get("id") {
        Some(id_str) => match ObjectId::parse_str(id_str) {
            Ok(object_id) => object_id,
            Err(_) => {
                return CustomError::BadRequestError("Invalid todo ID format".to_string())
                    .error_response()
            }
        },
        None => {
            return CustomError::BadRequestError("Todo ID not provided".to_string())
                .error_response()
        }
    };

    // Fetch the todo item
    match todo_service.get_todo(id, user_id).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo), // Return the todo item as JSON
        Ok(None) => HttpResponse::NotFound().body("Todo not found"), // Handle case where todo does not exist
        Err(e) => HttpResponse::InternalServerError().body(e),       // Handle any other
    }
}

pub async fn update_todo(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    todo_update: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    // Check for the Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        debug!("Authorization header: {:?}", auth_header);
    } else {
        debug!("No Authorization header found");
        return HttpResponse::Unauthorized().body("No Authorization header found");
    }

    // Extract user_id from token
    let user_id = match verify_token(&req).await {
        Ok(id) => match ObjectId::parse_str(&id) {
            Ok(object_id) => object_id,
            Err(_) => {
                return HttpResponse::BadRequest().body("Invalid user ID format in token");
            }
        },
        Err(_) => {
            return HttpResponse::Unauthorized().body("Invalid or missing token");
        }
    };

    // Extract the todo ID from the request path
    let id = match req.match_info().get("id") {
        Some(id_str) => match ObjectId::parse_str(id_str) {
            Ok(object_id) => object_id,
            Err(_) => return HttpResponse::BadRequest().body("Invalid todo ID format"),
        },
        None => return HttpResponse::BadRequest().body("Todo ID not provided"),
    };

    // Create a Todo struct from the UpdateTodoRequest
    // Fetch the existing todo item
    let existing_todo = match todo_service.get_todo(id.clone(), user_id.clone()).await {
        Ok(Some(todo)) => todo,
        Ok(None) => return HttpResponse::NotFound().body("Todo not found"),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    // Create a new Todo object based on the existing data and the update request
    let updated_todo = Todo {
        id: existing_todo.id,
        title: todo_update.title.clone().unwrap_or(existing_todo.title),
        description: todo_update
            .description
            .clone()
            .unwrap_or(existing_todo.description),
        completed: todo_update.completed.unwrap_or(existing_todo.completed),
        user_id: existing_todo.user_id,
    };

    // Call the service to update the todo
    match todo_service.update_todo(id, user_id, updated_todo).await {
        Ok(updated) => {
            if updated {
                HttpResponse::Ok().body("Todo updated successfully")
            } else {
                HttpResponse::NotFound().body("Todo not found or not updated")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
