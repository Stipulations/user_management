use chrono::{Duration, Utc};
use colored::Colorize;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{distributions::Alphanumeric, Rng};
use rusqlite::{params, Connection};
use serde::Serialize;
use tokio::task;

const JWT_SECRET: &[u8] = b"dontshare";

pub async fn init() {
    if let Err(e) = create_users_table().await {
        eprintln!("Error creating users table: {}", e.red());
    }

    match create_owner().await {
        Ok(password) => println!(
            "Owner login: owner:{} | {}",
            password.blue(),
            "SAVE YOUR PASSWORD".bright_red()
        ),
        Err(e) => println!("Error: {}", e.yellow()),
    }
}

async fn create_users_table() -> Result<(), String> {
    task::spawn_blocking(|| {
        let conn = Connection::open("database.db").map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                role TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

async fn create_owner() -> Result<String, String> {
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

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    role: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    role: String,
    id: i32,
    exp: usize,
}

pub async fn login_check(
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::open("database.db")?;

    let mut stmt = conn.prepare(
        "SELECT id, username, password, role FROM users WHERE username = ?1 AND password = ?2",
    )?;
    let user_iter = stmt.query_map(params![username, password], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            role: row.get(3)?,
        })
    })?;

    for user in user_iter {
        let user = user?;

        let claims = Claims {
            sub: user.username.clone(),
            role: user.role.clone(),
            id: user.id,
            exp: (Utc::now() + Duration::weeks(1)).timestamp() as usize, // Use chrono for expiration time
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET),
        )?;

        return Ok(token);
    }

    Err("Invalid username or password".into())
}
