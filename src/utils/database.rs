use sea_orm::{DatabaseConnection, DbErr};

pub fn create_database_url(
    protocol: &str,
    user: &str,
    password: &str,
    host: &str,
    port: &u16,
    database: &str,
) -> String {
    format!("{protocol}://{user}:{password}@{host}:{port}/{database}",
        protocol = protocol,
        user = user,
        password = password,
        host = host,
        port = port,
        database = database,
    )
}

pub async fn create_database_connection(
    protocol: &str,
    user: &str,
    password: &str,
    host: &str,
    port: &u16,
    database: &str,
) -> Result<DatabaseConnection, DbErr> {
    println!("Creating database connection url...");
    let db_url = create_database_url(
        protocol,
        user,
        password,
        host,
        port,
        database,
    );
    let outcome = sea_orm::Database::connect(&db_url).await;
    println!("Database connection created using {}", &db_url);

    return outcome;
}