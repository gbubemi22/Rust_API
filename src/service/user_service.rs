use crate::model::user_model::User;
use crate::utils::error::CustomError;
use crate::utils::model::LoginRequests;
use crate::utils::{hashing, password_validation};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};
use serde::Serialize;

pub struct UserService {
    collection: Collection<User>,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct Claims {
    id: String,
    exp: usize,
}

impl UserService {
    pub fn new(client: &Client) -> Self {
        let collection = client.database("Rust_PRo").collection("users");
        UserService { collection }
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<ObjectId, String> {
        // Check if email already exists
        if self.email_exists(&email).await.map_err(|e| e.to_string())? {
            // return Err(CustomError::ConflictError("Email already exists".to_string()).to_string());
            return Err("Email already exists".to_string());
        }
        // Check if username already exists
        if self
            .username_exists(&username)
            .await
            .map_err(|e| e.to_string())?
        {
            // return Err(
            //     CustomError::ConflictError("Username already exists".to_string()).to_string(),
            // );
            return Err("Username already exists".to_string());
        }

        password_validation::validate_password(&password)
            .map_err(|e| CustomError::BadRequestError(e.to_string()))
            .ok();

        // Hash the password
        let hashed_password = hashing::hash_password(&password).map_err(|e| e.to_string())?;

        // Create new user
        let new_user = User {
            id: None,
            username,
            email,
            password: hashed_password,
        };

        // Insert the user
        let result = self
            .collection
            .insert_one(new_user, None)
            .await
            .map_err(|e| e.to_string())?;

        // Return the inserted ID
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| "Failed to get inserted ID".to_string())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, mongodb::error::Error> {
        let count = self
            .collection
            .count_documents(doc! { "email": email }, None)
            .await?;
        Ok(count > 0)
    }

    async fn username_exists(&self, username: &str) -> Result<bool, mongodb::error::Error> {
        let count = self
            .collection
            .count_documents(doc! { "username": username }, None)
            .await?;
        Ok(count > 0)
    }

    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, CustomError> {
        let user = self
            .collection
            .find_one(doc! { "username": username }, None)
            .await
            .map_err(|_| CustomError::InternalServerError("Database error".to_string()))?
            .ok_or_else(|| CustomError::UnauthorizedError("Invalid credentials".to_string()))?;

        if !hashing::verify_password(password, &user.password)
            .map_err(|_| CustomError::InternalServerError("Invalid credentials".to_string()))?
        {
            return Err(CustomError::UnauthorizedError(
                "Invalid credentials".to_string(),
            ));
        }

        Ok(user)
    }
    pub async fn login_fn(&self, login_data: LoginRequests) -> Result<String, CustomError> {
        // Authenticate user
        let user = self
            .authenticate_user(&login_data.username, &login_data.password)
            .await?;

        // Generate JWT token
        let claims = Claims {
            id: user.id.unwrap().to_string(),
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        )
        .map_err(|_| CustomError::InternalServerError("Token generation failed".to_string()))?;

        Ok(token)
    }

    // ... other methods ...
}
