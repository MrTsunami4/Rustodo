use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Todo {
    id: Uuid,
    content: String,
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
        }
    }

    pub const fn from_db(id: Uuid, content: String) -> Self {
        Self { id, content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub const fn id(&self) -> &Uuid {
        &self.id
    }
}
