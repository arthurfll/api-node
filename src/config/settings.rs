use std::env;
use sqlx::mysql::MySqlPool;


pub fn address() -> String {

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());

    format!("{}:{}", ip, port)
}

pub async fn conn_db() -> MySqlPool {
    let database_url = env::var("MYSQL_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create a database connection pool")
}
