// External dependencies
use sea_orm::DatabaseConnection;

// Application level dependencies
use crate::{models::settings::Settings, utils};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
}

impl AppState {
    pub async fn from_settings(settings: &Settings) -> Self {
        Self {
            db_connection: utils::database::create_database_connection(
                &settings.database.protocol,
                &settings.database.user,
                &settings.database.password,
                &settings.database.host,
                &settings.database.port,
                &settings.database.database,
            )
            .await
            .unwrap(),
        }
    }
}
