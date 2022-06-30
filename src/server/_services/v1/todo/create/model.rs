// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::Component;

// Application level dependencies
use super::super::model::Todo;

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoCreatePayload {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoCreateResponse {
    pub todo: Todo,
}
