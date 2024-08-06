use crate::db;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(login_request: web::Json<LoginRequest>) -> impl Responder {
    let (username, password) = (
        login_request.username.clone(),
        login_request.password.clone(),
    );

    match db::login_check(&username, &password).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

pub async fn profile() -> impl Responder {
    HttpResponse::Ok().body("Profile endpoint")
}
