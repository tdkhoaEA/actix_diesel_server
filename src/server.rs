use actix_web::{App, HttpServer};
use std::env;

use crate::handlers;

pub async fn start_server() -> std::io::Result<()> {
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
        let app = App::new()
            .configure(|config| handlers::packages::router::mount(config))
            .configure(|config| handlers::home::router::mount(config));
        app
    })
    .bind((bind_to, port))?
    .run()
    .await
}
