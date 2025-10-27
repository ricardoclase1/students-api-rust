use sqlx::SqlitePool;
use std::fs;

pub async fn init_db(database_url: &str) -> SqlitePool {
    // For in-memory database, we need to handle it differently
    let connection_url = if database_url == ":memory:" {
        "sqlite::memory:".to_string()
    } else {
        database_url.to_string()
    };

    let pool = SqlitePool::connect(&connection_url).await.expect("DB connect failed");
    let sql = fs::read_to_string("migrations/01_db_init.sql").expect("read migration");
    sqlx::query(&sql).execute(&pool).await.expect("apply migration");
    pool
}
