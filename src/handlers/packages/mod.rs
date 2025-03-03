pub mod controller;
pub mod model;

use actix_web::web::{post, resource, scope, ServiceConfig};

pub fn mount(config: &mut ServiceConfig) {
    // println!("Packages mount");
    config.service(
        scope("/packages/")
            // Index
            .service(resource("").to(controller::index))
            // New
            .service(resource("/new").to(controller::new))
            // Create
            .service(resource("/create").route(post().to(controller::create)))
            // Update Downloads Count
            .service(resource("/{id}/download").to(controller::download))
    );
}
