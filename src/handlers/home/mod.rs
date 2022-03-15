pub mod controller;

use actix_web::web::{resource, scope, ServiceConfig};

pub fn mount(config: &mut ServiceConfig) {
    config.service(
        scope("/")
            // Home page
            .service(resource("").to(controller::index))
    );
}
