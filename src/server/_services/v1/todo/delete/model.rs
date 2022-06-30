// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::{Component, IntoParams};

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct TodoDeleteParams {
    #[param(example = 1)]
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoDeleteResponse {
    pub id: i32,
}
