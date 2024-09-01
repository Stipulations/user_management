use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug)]
struct User {
    uid: i32,
    username: String,
    role: String,
}

#[derive(Serialize)]
struct Claims {
    username: String,
    role: String,
    uid: i32,
    exp: usize,
}

pub async fn login_check(
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let jwt_secret =
        env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not found in .env".to_string())?;
    let conn = Connection::open("database.db")?;

    let mut stmt = conn.prepare(
        "SELECT id, username, password, role FROM users WHERE username = ?1 AND password = ?2",
    )?;
    let user_iter = stmt.query_map(params![username, password], |row| {
        Ok(User {
            uid: row.get(0)?,
            username: row.get(1)?,
            role: row.get(3)?,
        })
    })?;

    for user in user_iter {
        let user = user?;

        let claims = Claims {
            username: user.username.clone(),
            role: user.role.clone(),
            uid: user.uid,
            exp: (Utc::now() + Duration::weeks(1)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )?;

        return Ok(token);
    }

    Err("Invalid username or password".into())
}
