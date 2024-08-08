use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusqlite::{params, Connection};
use serde::Serialize;

const JWT_SECRET: &[u8] = b"dontshare";


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
            exp: (Utc::now() + Duration::weeks(1)).timestamp() as usize,
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