use actix_web::{HttpResponse, Responder};

use crate::middleware::session_checker::session_checker9000;

pub async fn profile() -> impl Responder {
    session_checker9000();
    HttpResponse::Ok().body("Profile endpoint")
}