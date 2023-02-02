use self::entity::{node, user};
use crate::db::migrations::Migrator;
use anyhow::Error;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, ConnectOptions, Database as SeaOrmDatabase};
use sea_orm_migration::MigratorTrait;
use std::env;

pub mod entity;
pub mod migrations;

const DB_PATH_ENV_NAME: &str = "DATABASE_PATH";

pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn load_default() -> Result<Self, Error> {
        if let Ok(db_file) = env::var(DB_PATH_ENV_NAME) {
            Database::new(format!("sqlite://{db_file}?mode=rwc")).await
        } else {
            eprintln!("Warning: Using in-memory database. Data will be lost when server stops");
            Database::new_in_memory().await
        }
    }

    /// Creates a new in-memory database. Contents will be lost when the last connection is closed.
    pub async fn new_in_memory() -> Result<Self, Error> {
        Database::new("sqlite::memory:").await
    }

    async fn new<C: Into<ConnectOptions>>(db_url: C) -> Result<Self, Error> {
        let db = SeaOrmDatabase::connect(db_url).await?;
        Migrator::up(&db, None).await?;
        Ok(Self { db })
    }

    pub async fn insert_node(&self, latitude: f64, longitude: f64) -> Result<u64, Error> {
        let new_node = node::ActiveModel {
            latitude: ActiveValue::Set(latitude),
            longitude: ActiveValue::Set(longitude),
            ..Default::default()
        };

        let res = node::Entity::insert(new_node).exec(&self.db).await?;

        Ok(res.last_insert_id)
    }

    pub async fn insert_user(&self, email: String, password_hash: String) -> Result<(), Error> {
        let new_user = user::ActiveModel {
            email: ActiveValue::Set(email.to_lowercase()),
            password_hash: ActiveValue::Set(password_hash),
            ..Default::default()
        };

        match user::Entity::insert(new_user).exec(&self.db).await {
            Err(DbErr::Exec(_)) => {
                return Err(Error::msg("Email already registered"));
            }
            other => {
                other?;
            }
        }
        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<user::Model, Error> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email.to_lowercase()))
            .one(&self.db)
            .await?
            .ok_or(Error::msg("Could not find user"))
    }

    pub async fn get_nodes(
        &mut self,
        owner_email: Option<String>,
    ) -> Result<Vec<node::Model>, Error> {
        let filter = if let Some(owner_email) = owner_email {
            node::Column::Owner.eq(owner_email)
        } else {
            node::Column::Owner.is_null()
        };

        Ok(node::Entity::find().filter(filter).all(&self.db).await?)
    }

    pub async fn set_node_owner(
        &mut self,
        node_id: u64,
        owner_email: Option<String>,
    ) -> Result<(), Error> {
        node::Entity::update(node::ActiveModel {
            owner: ActiveValue::Set(owner_email),
            ..Default::default()
        })
        .filter(node::Column::Id.eq(node_id))
        .exec(&self.db)
        .await?;
        Ok(())
    }
}
