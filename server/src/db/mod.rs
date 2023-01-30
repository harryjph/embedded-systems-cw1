use std::env;
use std::error::Error;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Database as SeaOrmDatabase};
use sea_orm_migration::MigratorTrait;
use crate::db::migrations::Migrator;
use self::entities::{prelude::*, *};

pub mod migrations;
pub mod entities;

const DB_PATH_ENV_NAME: &str = "DATABASE_PATH";

pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let db_url = if let Ok(db_file) = env::var(DB_PATH_ENV_NAME) {
            format!("sqlite://{db_file}?mode=rwc")
        } else {
            "sqlite::memory:".to_string()
        };
        let db = SeaOrmDatabase::connect(db_url).await?;
        Migrator::up(&db, None).await?;

        Ok(Self { db })
    }

    pub async fn insert_node(
        &mut self,
        latitude: f64,
        longitude: f64,
    ) -> Result<(), Box<dyn Error>> {
        let new_node = node::ActiveModel {
            latitude: ActiveValue::Set(latitude),
            longitude: ActiveValue::Set(longitude),
            ..Default::default()
        };

        let res = Node::insert(new_node).exec(&self.db).await?;

        Ok(())
    }

    pub async fn get_nodes(&mut self) -> Result<(), Box<dyn Error>> {
        let nodes: Vec<node::Model> = Node::find().all(&self.db).await?;

        for node in nodes {
            println!("Longitude: {}, Latitude: {}", node.longitude, node.latitude);
        }

        Ok(())
    }
}
