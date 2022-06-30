// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::{Component, IntoParams};

// Application level dependencies
use super::super::model::Todo;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct TodoGetParams {
    #[param(example = 1)]
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoGetResponse {
    pub todo: Todo,
}
