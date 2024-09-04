use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::model::todo_model::Todo;

pub struct TodoService {
    collection: Collection<Todo>,
}

impl TodoService {
    pub fn new(client: &Client) -> Self {
        let collection = client.database("Rust_PRo").collection("todos");
        TodoService { collection }
    }

    pub async fn create_todo(
        &self,
        title: String,
        description: String,
        user_id: ObjectId,
    ) -> Result<ObjectId, String> {
        let todo = Todo {
            id: None,
            title,
            description,
            completed: false,
            user_id,
        };

        let insert_result = self
            .collection
            .insert_one(todo, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(insert_result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| "Failed to get inserted id".to_string())?)
    }

    pub async fn list_todos(&self, user_id: ObjectId) -> Result<Vec<Todo>, String> {
        println!("Querying todos for user_id: {}", user_id);

        // Check how many documents match the user_id
        let count = self
            .collection
            .count_documents(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| e.to_string())?;
        println!("Number of todos for user_id {}: {}", user_id, count);

        let mut cursor = self
            .collection
            .find(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| e.to_string())?;

        let mut todos = Vec::new();
        while let Some(todo) = cursor.try_next().await.map_err(|e| e.to_string())? {
            todos.push(todo);
        }

        Ok(todos)
    }

    pub async fn get_todo(&self, id: ObjectId, user_id: ObjectId) -> Result<Option<Todo>, String> {
        self.collection
            .find_one(doc! { "_id": id, "user_id": user_id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_todo(
        &self,
        id: ObjectId,
        user_id: ObjectId,
        todo: Todo,
    ) -> Result<bool, String> {
        let update_result = self
            .collection
            .update_one(
                doc! { "_id": id, "user_id": user_id },
                doc! { "$set": todo.to_doc() },
                None,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(update_result.modified_count == 1)
    }

    pub async fn delete_todo(&self, id: ObjectId, user_id: ObjectId) -> Result<bool, String> {
        let delete_result = self
            .collection
            .delete_one(doc! { "_id": id, "userId": user_id }, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(delete_result.deleted_count == 1)
    }
}
