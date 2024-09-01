use actix_web::{web, App, HttpServer};
use colored::Colorize;

mod db;
mod middleware;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init().await;

    println!("Webserver starting at {}", "127.0.0.1:8080".red());
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(routes::login::login))
            .route("/profile", web::get().to(routes::profile::profile))
            .route("/logout", web::get().to(routes::logout::logout))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
