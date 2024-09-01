use actix_web::http::header::AUTHORIZATION;
use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::middleware::session_checker::session_checker9000;

pub async fn profile(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get(AUTHORIZATION);

    let token = match auth_header {
        Some(header_value) => {
            if let Ok(header_str) = header_value.to_str() {
                if header_str.starts_with("Bearer ") {
                    Some(&header_str[7..])
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    };

    match token {
        Some(jwt) => match session_checker9000(jwt, "decode") {
            Ok(Some(claims)) => HttpResponse::Ok().json(claims),
            Ok(None) => HttpResponse::Unauthorized().body("Invalid token"),
            Err(e) => {
                let response = match e.as_str() {
                    "Token has expired" => HttpResponse::Unauthorized().body("Token has expired"),
                    "Token is invalid" => HttpResponse::Unauthorized().body("Token is invalid"),
                    "Signature is invalid" => {
                        HttpResponse::Unauthorized().body("Invalid signature")
                    }
                    _ => HttpResponse::InternalServerError().body("Internal server error"),
                };
                response
            }
        },
        None => HttpResponse::BadRequest().body("Authorization header missing or invalid"),
    }
}
