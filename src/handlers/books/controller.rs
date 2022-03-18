use actix_web::{HttpResponse, web::Form, web::Path};
use tera::Context;

use mainlib::renderer;
use mainlib::database;

use super::model::*;


pub async fn index() -> HttpResponse {      
  println!("Books index");      
  let connection = database::establish_connection();

  let books = Book::all(&connection);

  // for i in books.iter(){
  //   Book::show(i);
  // }
  
  //println!("This the package recieve {}", Book::t(&books[0]));
  let mut context = Context::new();
  context.insert("books", &books  );

  renderer::render(200, "books/index.html", &context)
}
