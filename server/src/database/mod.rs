use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub mod models;
pub mod schema;

pub struct Database<C> {
    connection: C,
}

impl<C: Connection> Database<C> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        Ok(Self {
            connection: C::establish(&database_url)?,
        })
    }
}

impl Database<SqliteConnection> {
    pub fn new_sqlite() -> Result<Self, Box<dyn Error>> {
        Self::new()
    }
}
