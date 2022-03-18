use diesel::{Queryable, Identifiable, Insertable,AsChangeset};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use mainlib::schema::books::dsl::*;
use mainlib::schema::books;


#[derive(Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
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
  pub fn create<'a>(record: &NewBook,connection: &PgConnection)-> Book{
    diesel::insert_into(books::table)
    .values(record)
    .get_result(connection)
    .expect("Error saving")

  }
  // pub fn update(self:&Self, connection: &PgConnection)-> Book{
  //   self.save_changes(connection).unwrap()
  // }
  


  //TODO:
  // find, update, filter, add more values
}
#[derive(Insertable, Deserialize)]
#[table_name="books"]

pub struct NewBook{
  pub title: String,
  pub description: String,
  pub price: i32,
  pub rating:i32
}

//

