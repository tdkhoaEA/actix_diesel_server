use actix_web::{web, App, HttpServer};
use std::env;

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_to = env::var("BIND_TO").expect("BIND_TO must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/packages/", web::get().to(handlers::packages::controller::index))
            .route("/packages/new", web::get().to(handlers::packages::controller::new))
    })
    .bind((bind_to, port))?
    .run()
    .await
}