use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: ObjectId,
}

impl Todo {
    pub fn new(title: String, description: String, user_id: ObjectId) -> Self {
        Todo {
            id: None,
            title,
            description,
            completed: false,
            user_id,
        }
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn to_doc(&self) -> mongodb::bson::Document {
        doc! {
            "title": &self.title,
            "description": &self.description,
            "completed": self.completed,
            "userId": &self.user_id,
        }
    }
}
