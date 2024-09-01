use actix_web::{HttpResponse, Responder};

pub async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout endpoint")
}
