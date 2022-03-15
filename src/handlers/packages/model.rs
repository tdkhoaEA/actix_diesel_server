use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use mainlib::schema::packages::dsl::*;
use mainlib::schema::packages;

/// A Package Object.
#[derive(Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub downloads_count: i32
}

impl Package {

    pub fn all(connection: &PgConnection) -> Vec<Self> {
        let results = packages
        .load::<Package>(connection)
        .expect("Error loading packages");
        
        results
    }

    pub fn create<'a>(record: &NewPackage, connection: &PgConnection) -> Package {
        diesel::insert_into(packages::table)
            .values(record)
            .get_result(connection)
            .expect("Error saving new package")
    }

    // pub fn update(self: &Self, connection: &PgConnection) -> Package {
    //     self.save_changes(connection).unwrap()
    // }

    // pub fn find(uid: i32, connection: &PgConnection) -> Package {
    //     return packages::table.find(uid).first::<Package>(connection).expect("Error loading package");
    // }
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub description: &'a str
}