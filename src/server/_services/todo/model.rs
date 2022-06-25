// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::Component;

// Internal dependencies
use entity::todo::Model as TodoModel;

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn from_entity(entity: &TodoModel) -> Self {
        Self {
            id: entity.id,
            title: entity.title.clone(),
            completed: entity.completed_at.is_some(),
        }
    }
}
