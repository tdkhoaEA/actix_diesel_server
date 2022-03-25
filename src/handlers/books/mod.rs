pub mod controller;
pub mod model;

use actix_web::web::{post,resource,scope,ServiceConfig};


pub fn mount(config: &mut ServiceConfig) {
  // println!("Books mount");
  config.service(
      scope("/books/")
          // Index
          .service(resource("").to(controller::index))
          // // New
          .service(resource("/search/{name}").to(controller::search))
          // Create
          .service(resource("/create").route(post().to(controller::create)))
          // // Update Downloads Count
          // .service(resource("/{id}/download").to(controller::download))
  );
}
