use crate::model::user_model::User;
use crate::utils::hashing;

use mongodb::{Client, Collection};
use mongodb::bson::{doc, oid::ObjectId};


pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(client: &Client) -> Self {
        let collection = client.database("Rust_PRo").collection("users");
        UserService { collection }
    }

    pub async fn create_user(&self, username: String, email: String, password: String) -> Result<ObjectId, String> {
        // Check if email already exists
        if self.email_exists(&email).await.map_err(|e| e.to_string())? {
            return Err("Email already exists".to_string());
        }

        // Check if username already exists
        if self.username_exists(&username).await.map_err(|e| e.to_string())? {
            return Err("Username already exists".to_string());
        }

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
        let result = self.collection.insert_one(new_user, None)
            .await
            .map_err(|e| e.to_string())?;

        // Return the inserted ID
        result.inserted_id.as_object_id()
            .ok_or_else(|| "Failed to get inserted ID".to_string())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, mongodb::error::Error> {
        let count = self.collection.count_documents(doc! { "email": email }, None).await?;
        Ok(count > 0)
    }

    async fn username_exists(&self, username: &str) -> Result<bool, mongodb::error::Error> {
        let count = self.collection.count_documents(doc! { "username": username }, None).await?;
        Ok(count > 0)
    }

    // ... other methods ...
}