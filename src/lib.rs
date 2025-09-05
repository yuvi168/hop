use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewPath, Path};

pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert(new_path: String) {
    use crate::schema::paths;
    let path = NewPath {path: new_path};
    diesel::insert_into(paths::table).values(&path)
        .execute(&mut establish_connection())
        .expect("Error saving new path");
}

pub fn retrieve(directory: String) -> String {
    use self::schema::{ self, paths::dsl::*};
    
    let result = paths
        .filter(path.like(directory))
        .select(Path::as_select())
        .first(&mut establish_connection())
        .optional()
        .expect("Couldn't find the path with the queried directory");
    result.unwrap().path
}

