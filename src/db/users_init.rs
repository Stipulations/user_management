use rusqlite::Connection;
use tokio::task;

pub async fn create_users_table() -> Result<(), String> {
    task::spawn_blocking(|| {
        let conn = Connection::open("database.db").map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                role TEXT NOT NULL,
                session TEXT
            )",
            [],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
