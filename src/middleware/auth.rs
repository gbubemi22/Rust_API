use crate::utils::error::CustomError;
use actix_web::{Error, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::debug;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: usize,
}

pub async fn verify_token(req: &HttpRequest) -> Result<String, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "));

        debug!("Extracted token: {:?}", token);

        
    match token {
        Some(token) => {
            let secret = env::var("JWT_SECRET").map_err(|_| {
                CustomError::UnauthorizedError("JWT_SECRET must be set".to_string())
            })?;
            let key = DecodingKey::from_secret(secret.as_bytes());
            let validation = Validation::default();

            match decode::<Claims>(token, &key, &validation) {
                Ok(token_data) => Ok(token_data.claims.id),
                Err(_) => Err(CustomError::UnauthorizedError("Invalid token".to_string()).into()),
            }
        }
        None => Err(CustomError::UnauthorizedError(
            "Authorization header is missing or invalid".to_string(),
        )
        .into()),
    }
}

// pub async fn verify_token(req: ServiceRequest, _credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let token = req
//         .headers()
//         .get("Authorization")
//         .and_then(|header| header.to_str().ok())
//         .and_then(|auth_header| auth_header.strip_prefix("Bearer "));

//     match token {
//         Some(token) => {
//             let secret = env::var("JWT_SECRET").map_err(|_| CustomError::UnauthorizedError("JWT_SECRET must be set".to_string()))?;
//             let key = DecodingKey::from_secret(secret.as_bytes());
//             let validation = Validation::default();

//             match decode::<Claims>(token, &key, &validation) {
//                 Ok(token_data) => {
//                     req.extensions_mut().insert(token_data.claims.id);
//                     Ok(req)
//                 }
//                 Err(_) => Err(CustomError::UnauthorizedError("Invalid token".to_string()).into()),
//             }
//         }
//         None => Err(CustomError::UnauthorizedError("Authorization header is missing or invalid".to_string()).into()),
//     }
// }

// ... rest of your code

// pub fn verify_token(token: &str, expected_id: &str) -> Result<Claims, actix_web::Error> {
//      let secret = std::env::var("ACCESS_TOKEN_SECRET")
//          .expect("ACCESS_TOKEN_SECRET must be set");

//      match decode::<Claims>(
//          token,
//          &DecodingKey::from_secret(secret.as_bytes()),
//          &Validation::default(),
//      ) {
//          Ok(token_data) => {
//              if token_data.claims.id == expected_id {
//                  Ok(token_data.claims)
//              } else {
//                  Err(ErrorUnauthorized("Invalid token: ID mismatch"))
//              }
//          }
//          Err(err) => {
//              match err.kind() {
//                  jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
//                      Err(ErrorUnauthorized("Token has expired"))
//                  }
//                  _ => Err(ErrorUnauthorized("Invalid token"))
//              }
//          }
//      }
//  }
