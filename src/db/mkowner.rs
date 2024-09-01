use rand::{distributions::Alphanumeric, Rng};
use rusqlite::{params, Connection};
use tokio::task;

pub async fn create_owner() -> Result<String, String> {
    task::spawn_blocking(|| {
        let conn = Connection::open("database.db").map_err(|e| e.to_string())?;

        let owner_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM users WHERE id = 1)",
                [],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        if owner_exists {
            return Err("Owner already exists".into());
        }

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        conn.execute(
            "INSERT INTO users (username, password, role) VALUES (?1, ?2, ?3)",
            params!["owner", password, "owner"],
        )
        .map_err(|e| e.to_string())?;

        Ok(password)
    })
    .await
    .map_err(|e| e.to_string())?
}
