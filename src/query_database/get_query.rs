use std::error::Error;
use diesel::prelude::*;
use crate::models::Path;

pub fn get_path(dir_name: String) -> Result<String, Box<dyn Error>> {
    use crate::schema::paths::dsl::*;
    let result = paths
        .filter(path.like(dir_name))
        .select(Path::as_select())
        .first(&mut crate::db_connection::establish_connection())
        .optional()?
        .expect("can't find the path with dir_name");
    Ok(result.path)
}
