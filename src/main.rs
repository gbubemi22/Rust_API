use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

mod middleware;
use middleware::error_handler::handle_error;
use middleware::not_found::not_found;
use serde_json::json;
use service::user_service::UserService;

mod controller;
mod database;
mod model;
mod routes;
mod service;
mod utils;



#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Welcome to my Rust web-Server",
        "httpStatusCode": StatusCode::OK.as_u16(),
        "service": std::env::var("SERVICE_NAME").unwrap_or_else(|_| "Unknown".to_string()),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger with environment variable support
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Log the server start
    info!("Starting server on http://localhost:5001");

    let mongo_client = database::connect_to_mongo()
        .await
        .expect("Failed to connect to MongoDB");

    
 // Create UserService
 let user_service = web::Data::new(UserService::new(&mongo_client));
 
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(mongo_client.clone()))
            .app_data(user_service.clone())
            .configure(routes::router::config)
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::NOT_FOUND, not_found)
                    .default_handler(handle_error),
            )
            //.app_data(db_data.clone())
            .service(default)
    })
    .bind(("localhost", 5001))?
    .run()
    .await?;

    // Log after server has started (this line will only be reached when the server shuts down)
    info!("Server has stopped");

    Ok(())
}
