// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::{Component, IntoParams};

// Application level dependencies
use super::super::model::Todo;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct TodoUpdateParams {
    #[param(example = 1)]
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoUpdatePayload {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoUpdateResponse {
    pub todo: Todo,
}
