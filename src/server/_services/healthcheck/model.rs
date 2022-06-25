// External dependencies
use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
#[component(example = "Available")]
pub enum HealthCheckStatus {
    Available = 1,
    Unavailable = 2,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
#[component(example = json!({
    "status": "Available",
}))]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}
