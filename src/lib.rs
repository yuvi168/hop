use std::env;
use std::fs;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use regex::Regex;
use dotenvy::dotenv;
use diesel::prelude::*;
use self::models::{NewPath, Path};
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};

#[cfg(test)]
mod tests;

pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn retrieve_zsh_history() -> Result<(), Box<dyn Error>> {
    let zsh_history_path = format!(
        "/home/{}/.zsh_history",
        users::get_current_username()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let source: Vec<u8> = fs::read(zsh_history_path)?;
    let zsh_history = OsStr::from_bytes(&source[..])
        .to_string_lossy();
    let  re = Regex::new(r"cd\s*([^:\s]+)").unwrap();
    let mut paths: Vec<&str> = Vec::new();
    for (_, [path]) in re.captures_iter(&zsh_history).map(|c| c.extract()) {
        let path = path.trim();
        if !paths.contains(&path) {
            paths.push(&path);
        }
    }
    for path in paths {
        insert(path.to_string());
    }
    Ok(())
}

pub fn insert(new_path: String) {
    use crate::schema::paths;
    let path = NewPath {path: new_path};
    diesel::insert_into(paths::table).values(&path)
        .execute(&mut establish_connection())
        .expect("Error saving new path");
}

pub fn get_path(dir_name: String) -> Result<String, Box<dyn Error>> {
    use self::schema::paths::dsl::*;
    let result = paths
        .filter(path.like(dir_name))
        .select(Path::as_select())
        .first(&mut establish_connection())
        .optional()?
        .expect("can't find the path with dir_name");
    Ok(result.path)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration() {
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
