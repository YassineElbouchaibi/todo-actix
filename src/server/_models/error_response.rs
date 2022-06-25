use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}
