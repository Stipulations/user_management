use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};
use serde_json::Value;
use std::env;

pub fn session_checker9000(jwt: &str, task: &str) -> Result<Option<Value>, String> {
    dotenv::dotenv().ok();

    let jwt_secret =
        env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not found in .env".to_string())?;

    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    match task {
        "validate" => {
            match decode::<Value>(
                jwt,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &validation,
            ) {
                Ok(_) => Ok(Some(serde_json::json!(true))),
                Err(_) => Ok(Some(serde_json::json!(false))),
            }
        }
        "decode" => {
            match decode::<Value>(
                jwt,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &validation,
            ) {
                Ok(token_data) => Ok(Some(token_data.claims)),
                Err(err) => {
                    let error_message = match err.kind() {
                        ErrorKind::ExpiredSignature => "Token has expired",
                        ErrorKind::InvalidToken => "Token is invalid",
                        ErrorKind::InvalidSignature => "Signature is invalid",
                        _ => "Token decoding failed",
                    };
                    Err(error_message.to_string())
                }
            }
        }
        _ => Err("Invalid task".to_string()),
    }
}
