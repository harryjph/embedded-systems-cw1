use std::env;
use anyhow::Error;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Database as SeaOrmDatabase};
use sea_orm_migration::MigratorTrait;
use crate::db::migrations::Migrator;
use self::entity::{node, user};

pub mod migrations;
pub mod entity;

const DB_PATH_ENV_NAME: &str = "DATABASE_PATH";

pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        let db_url = if let Ok(db_file) = env::var(DB_PATH_ENV_NAME) {
            format!("sqlite://{db_file}?mode=rwc")
        } else {
            eprintln!("Warning: Using in-memory database. Data will be lost when server stops");
            "sqlite::memory:".to_string()
        };
        let db = SeaOrmDatabase::connect(db_url).await?;
        Migrator::up(&db, None).await?;

        Ok(Self { db })
    }

    pub async fn insert_node(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<u64, Error> {
        let new_node = node::ActiveModel {
            latitude: ActiveValue::Set(latitude),
            longitude: ActiveValue::Set(longitude),
            ..Default::default()
        };

        let res = node::Entity::insert(new_node).exec(&self.db).await?;

        Ok(res.last_insert_id)
    }

    pub async fn insert_user(
        &self,
        email: String,
        password_hash: String,
    ) -> Result<(), Error> {
        let new_user = user::ActiveModel {
            email: ActiveValue::Set(email.to_lowercase()),
            password_hash: ActiveValue::Set(password_hash),
            ..Default::default()
        };

        user::Entity::insert(new_user).exec(&self.db).await?;
        Ok(())
    }

    pub async fn get_user(&self, email: String) -> Result<user::Model, Error> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email.to_lowercase()))
            .one(&self.db).await?
            .ok_or(Error::msg("Could not find user"))
    }

    pub async fn get_nodes(&mut self) -> Result<(), Error> {
        let nodes: Vec<node::Model> = node::Entity::find().all(&self.db).await?;

        for node in nodes {
            println!("Longitude: {}, Latitude: {}", node.longitude, node.latitude);
        }

        Ok(())
    }
}
