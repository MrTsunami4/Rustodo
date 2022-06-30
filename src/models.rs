use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Todo {
    id: Uuid,
    content: String,
    completed: bool,
}

#[derive(Deserialize, Debug)]
pub struct TodoRequest {
    pub content: String,
}

impl Todo {
    pub fn new(content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.to_owned(),
            completed: false,
        }
    }

    pub const fn from_db(id: Uuid, content: String, completed: bool) -> Self {
        Self {
            id,
            content,
            completed,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub const fn id(&self) -> &Uuid {
        &self.id
    }
    pub const fn completed(&self) -> bool {
        self.completed
    }
}
