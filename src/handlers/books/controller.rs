use actix_web::{HttpResponse,HttpRequest, web::Form, web::Path};
use tera::Context;

use mainlib::renderer;
use mainlib::database;

use super::model::*;


pub async fn index() -> HttpResponse {      
  //println!("Books index");      
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


pub async fn create(form: Form<NewBook>) -> HttpResponse{
  println!("creating book");
  let object = form.into_inner();
  let connection= database::establish_connection();


  //function create have by the models
  _ = Book::create(&object,&connection);

  renderer::redirect("/books/")
}

///req: Path<String>
 pub async fn search(req : Path<String>)-> HttpResponse{
  println!("this name is {}",req);
  let mut context = Context::new();
  context.insert("search",&req.into_inner());
  renderer::render(200,"books/search.html",&context)
 }