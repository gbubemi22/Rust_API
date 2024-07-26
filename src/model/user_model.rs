use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::utils::hashing;
use bcrypt;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let hashed_password = match hashing::hash_password(&password) {
            Ok(hashed) => hashed,
            Err(e) => panic!("Failed to hash password: {}", e),
        };
        User {
            id: None,
            username,
            email,
            password: hashed_password,
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        hashing::verify_password(password, &self.password)
    }
}
