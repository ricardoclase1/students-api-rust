use sqlx::SqlitePool;
use std::fs;

pub async fn init_db(database_url: &str) -> SqlitePool {
    let pool = SqlitePool::connect(database_url).await.expect("DB connect failed");
    let sql = fs::read_to_string("migrations/01_db_init.sql").expect("read migration");
    sqlx::query(&sql).execute(&pool).await.expect("apply migration");
    pool
}
