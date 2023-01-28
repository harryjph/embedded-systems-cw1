use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub mod models;
pub mod schema;

use self::models::{NewNode, Node};

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        Ok(Self {
            connection: SqliteConnection::establish(&database_url)?,
        })
    }

    /// Example of how to insert into db
    pub fn insert_node(
        &mut self,
        longitude: f64,
        latitude: f64,
    ) -> Result<(), diesel::result::Error> {
        use schema::nodes;
        let new_node = NewNode {
            longitude,
            latitude,
        };

        diesel::insert_into(nodes::table)
            .values(&new_node)
            .execute(&mut self.connection)?;

        Ok(())
    }

    pub fn get_nodes(&mut self) -> Result<(), diesel::result::Error> {
        use schema::nodes::dsl::*;
        let nodes_vec = nodes.load::<Node>(&mut self.connection)?;

        for node in nodes_vec {
            println!("Longitude: {}, Latitude: {}", node.longitude, node.latitude);
        }

        Ok(())
    }
}
