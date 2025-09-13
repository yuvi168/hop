use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};

#[cfg(test)]
mod tests;

pub mod query_database;
mod models;
mod schema;
mod db_connection;
pub mod retrieve;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration() {
    let conn = &mut db_connection::establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
