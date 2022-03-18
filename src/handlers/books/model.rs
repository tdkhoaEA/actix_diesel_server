use diesel::{Queryable, Identifiable, Insertable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use mainlib::schema::books::dsl::*;
use mainlib::schema::books;


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Book{ // use in controller
  pub id: i32,
  pub title: String,
  pub description: String,
  pub price : i32,
  pub rating : i32,

}

impl Book{
  pub fn all(connection: &PgConnection) -> Vec<Self>{
    let results = books.load::<Book>(connection).expect("Error loading book");

    results
  }
  pub fn show(book: &Book){
    println!("All the information of the books:");
    println!("Book name: {}", book.title);
  }
  
}


