use diesel::prelude::*;
use crate::models::NewPath;

pub fn insert(new_path: String) {
    use crate::schema::paths;
    let path = NewPath {path: new_path};
    diesel::insert_into(paths::table).values(&path)
        .execute(&mut crate::db_connection::establish_connection())
        .expect("Error saving new path");
}
