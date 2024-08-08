use actix_web::{HttpResponse, Responder};

pub async fn profile() -> impl Responder {
    HttpResponse::Ok().body("Profile endpoint")
}