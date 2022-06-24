// External dependencies
use serde::Serialize;

#[derive(Serialize)]
pub enum HealthCheckStatus {
    Available = 1,
    Unavailable = 2,
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}
