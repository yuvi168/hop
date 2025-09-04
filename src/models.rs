use diesel::prelude::*;
use crate::schema::paths;

#[derive(Queryable, Selectable)]
#[diesel(table_name=paths)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Path {
   pub rowid: i32,
   pub path: String,
   pub score: i32,
}

#[derive(Insertable)]
#[diesel(table_name=paths)]
pub struct NewPath {
    pub path: String
}
