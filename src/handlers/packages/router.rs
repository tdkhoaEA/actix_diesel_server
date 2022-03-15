use actix_web::web::{post, resource, scope, ServiceConfig};

use super::controller;

pub fn mount(config: &mut ServiceConfig) {
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
