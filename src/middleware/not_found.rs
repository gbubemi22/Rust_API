use actix_web::{dev::ServiceResponse, Result, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;
use serde_json::json;

pub fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_response = HttpResponse::build(StatusCode::NOT_FOUND)
        .json(json!({
            "success": false,
            "message": "Route does not exist",
            "httpStatusCode": StatusCode::NOT_FOUND.as_u16(),
            "error": "NOT_FOUND_ERROR",
            "service": std::env::var("SERVICE_NAME").unwrap_or_else(|_| "Unknown".to_string()),
        }));
        let (req, _) = res.into_parts();
        let res = ServiceResponse::new(
            req,
            new_response.map_into_right_body()
        );
    
        Ok(ErrorHandlerResponse::Response(res))
}


// use actix_web::{HttpResponse, Result};
// use actix_web::http::StatusCode;
// use actix_web::dev::{ServiceResponse, ServiceRequest};
// use actix_web::body::BoxBody;
// use actix_web::middleware::ErrorHandlerResponse;
// use serde_json::json;

// pub fn not_found<B>(req: ServiceRequest, res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
//     let _ = res;
//     let new_response = HttpResponse::build(StatusCode::NOT_FOUND)
//         .content_type("application/json")
//         .json(json!({
//             "success": false,
//             "message": "Route does not exist",
//             "httpStatusCode": StatusCode::NOT_FOUND.as_u16(),
//             "error": "NOT_FOUND_ERROR",
//             "service": std::env::var("SERVICE_NAME").unwrap_or_else(|_| "Unknown".to_string()),
//         }));
    
//     let (req, _) = req.into_parts();
//     let res = ServiceResponse::new(
//         req,
//         new_response.map_into_right_body()
//     );

//     Ok(ErrorHandlerResponse::Response(res))
// }




// use actix_web::{HttpResponse, Result};
// use actix_web::http::StatusCode;
// use actix_web::dev::{ServiceResponse, ServiceRequest};
// use actix_web::body::BoxBody;
// use actix_web::middleware::ErrorHandlerResponse;
// use serde_json::json;

// pub fn not_found<B>(req: ServiceRequest, res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
//     let new_response = HttpResponse::build(StatusCode::NOT_FOUND)
//         .content_type("application/json")
//         .json(json!({
//             "success": false,
//             "message": "Route does not exist",
//             "httpStatusCode": StatusCode::NOT_FOUND.as_u16(),
//             "error": "NOT_FOUND_ERROR",
//             "service": std::env::var("SERVICE_NAME").unwrap_or_else(|_| "Unknown".to_string()),
//         }));
    
//     Ok(ErrorHandlerResponse::Response(
//         ServiceResponse::new(
//             req.into_parts().0,
//             new_response.map_into_boxed_body()
//         )
//     ))
// }


