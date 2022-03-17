use actix_web::{HttpResponse, web::Form, web::Path};
use tera::Context;

use mainlib::renderer;
use mainlib::database;

use super::model::*;


pub async fn index() -> HttpResponse {      
  println!("Books index");      
  let connection = database::establish_connection();

  // let all_packages = Package::all(&connection);

  let mut context = Context::new();
  //  context.insert("books", &all_packages);

  renderer::render(200, "books/index.html", &context)
}
