// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::{Component, IntoParams};

// Application level dependencies
use super::super::model::Todo;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct TodoListParams {
    #[param(style = Form, explode, allow_reserved, example = 1)]
    pub page: Option<usize>,
    #[param(style = Form, explode, allow_reserved, example = 10)]
    pub todos_per_page: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Component)]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
    pub page: usize,
    pub todos_per_page: usize,
    pub num_pages: usize,
}
