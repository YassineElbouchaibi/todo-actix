use serde::{Serialize};

#[derive(Serialize)]
pub enum Status {
    // Unknown = 0,
    Running = 1,
    // Dead = 2,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: Status
}