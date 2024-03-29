use self::entity::{node, user};
use crate::db::migrations::Migrator;
use anyhow::Error;
use chrono::{NaiveDateTime, Utc};
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

    /// Adds a new node with a blank initial config and returns its assigned ID
    pub async fn insert_node(&self) -> Result<u32, Error> {
        let new_node = node::ActiveModel {
            name: ActiveValue::Set(String::new()),
            owner: ActiveValue::Set(None),
            latitude: ActiveValue::Set(0.0),
            longitude: ActiveValue::Set(0.0),
            empty_distance_reading: ActiveValue::Set(1.0),
            full_distance_reading: ActiveValue::Set(0.0),
            fullness: ActiveValue::Set(0.0),
            temperature: ActiveValue::Set(0.0),
            humidity: ActiveValue::Set(0.0),
            data_last_updated: ActiveValue::Set(DateTimeUtc::from_utc(
                NaiveDateTime::from_timestamp_millis(0).unwrap(),
                Utc,
            )),
            ..Default::default()
        };

        let res = node::Entity::insert(new_node).exec(&self.db).await?;

        Ok(res.last_insert_id)
    }

    pub async fn insert_user<S: Into<String>>(
        &self,
        email: &str,
        password_hash: S,
    ) -> Result<(), Error> {
        let new_user = user::ActiveModel {
            email: ActiveValue::Set(email.to_lowercase()),
            password_hash: ActiveValue::Set(password_hash.into()),
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

    pub async fn get_user_last_email_time(
        &self,
        owner_email: &str,
    ) -> Result<Option<DateTimeUtc>, Error> {
        self.get_user(owner_email)
            .await
            .map(|model| model.last_email_time)
    }

    pub async fn set_user_last_email_time(
        &self,
        owner_email: &str,
        time: DateTimeUtc,
    ) -> Result<(), Error> {
        user::Entity::update(user::ActiveModel {
            email: ActiveValue::Unchanged(owner_email.to_lowercase()),
            last_email_time: ActiveValue::Set(Some(time)),
            ..Default::default()
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    /// Gets all nodes belonging to an owner.
    /// If the owner is None, returns all nodes without an owner
    pub async fn get_nodes(&self, owner_email: Option<&str>) -> Result<Vec<node::Model>, Error> {
        let filter = if let Some(owner_email) = owner_email {
            node::Column::Owner.eq(owner_email.to_lowercase())
        } else {
            node::Column::Owner.is_null()
        };

        Ok(node::Entity::find().filter(filter).all(&self.db).await?)
    }

    /// Gets a node by its id.
    /// Optionally filters by owner. If `owner_email` is Some, this will only return the
    /// node if its owner matches. If `owner_email` is None, this will not filter by owner.
    pub async fn get_node(
        &self,
        node_id: u32,
        owner_email: Option<&str>,
    ) -> Result<Option<node::Model>, Error> {
        let mut query = node::Entity::find().filter(node::Column::Id.eq(node_id));

        if let Some(owner_email) = owner_email {
            query = query.filter(node::Column::Owner.eq(owner_email.to_lowercase()));
        }

        Ok(query
            .filter(node::Column::Id.eq(node_id))
            .one(&self.db)
            .await?)
    }

    /// Sets the owner of a node.
    /// The node's current owner must be `old_owner_email` for the new owner to be set.
    pub async fn set_node_owner(
        &self,
        node_id: u32,
        old_owner_email: Option<&str>,
        owner_email: Option<&str>,
    ) -> Result<(), Error> {
        let old_owner_filter = if let Some(old_owner_email) = old_owner_email {
            node::Column::Owner.eq(Some(old_owner_email.to_lowercase()))
        } else {
            node::Column::Owner.is_null()
        };
        node::Entity::update(node::ActiveModel {
            id: ActiveValue::Unchanged(node_id as u32),
            owner: ActiveValue::Set(owner_email.map(str::to_lowercase)),
            ..Default::default()
        })
        .filter(old_owner_filter)
        .exec(&self.db)
        .await?;
        Ok(())
    }

    /// Sets node config.
    /// Optionally filters by owner. If `owner_email` is Some, this will only set the node
    /// config if its owner matches. If `owner_email` is None, this will not filter by owner.
    pub async fn set_node_config<S: Into<String>>(
        &self,
        node_id: u32,
        owner_email: Option<&str>,
        name: S,
        latitude: f64,
        longitude: f64,
        empty_distance_reading: f32,
        full_distance_reading: f32,
    ) -> Result<(), Error> {
        if empty_distance_reading <= full_distance_reading {
            return Err(Error::msg(
                "Empty distance reading must be greater than full distance reading",
            ));
        }

        let mut query = node::Entity::update(node::ActiveModel {
            id: ActiveValue::Unchanged(node_id as u32),
            name: ActiveValue::Set(name.into()),
            latitude: ActiveValue::Set(latitude),
            longitude: ActiveValue::Set(longitude),
            empty_distance_reading: ActiveValue::Set(empty_distance_reading),
            full_distance_reading: ActiveValue::Set(full_distance_reading),
            ..Default::default()
        });
        if let Some(owner_email) = owner_email {
            query = query.filter(node::Column::Owner.eq(owner_email.to_lowercase()));
        }
        query.exec(&self.db).await?;
        Ok(())
    }

    pub async fn set_node_data(
        &self,
        node_id: u32,
        fullness: f32,
        temperature: f32,
        humidity: f32,
    ) -> Result<(), Error> {
        if fullness < 0.0 || fullness > 1.0 {
            return Err(Error::msg("Fullness outside of range 0..1"));
        }
        node::Entity::update(node::ActiveModel {
            id: ActiveValue::Unchanged(node_id as u32),
            fullness: ActiveValue::Set(fullness),
            temperature: ActiveValue::Set(temperature),
            humidity: ActiveValue::Set(humidity),
            data_last_updated: ActiveValue::Set(Utc::now()),
            ..Default::default()
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::Database;

    #[tokio::test]
    async fn test_insert_and_get_user() {
        let db = Database::new_in_memory().await.unwrap();

        assert!(db.get_user(EMAIL).await.is_err());
        db.insert_user(EMAIL, PASSWORD_HASH).await.unwrap();
        let user = db.get_user(EMAIL).await.unwrap();
        assert_eq!(user.email, EMAIL.to_lowercase());
        assert_eq!(user.password_hash, PASSWORD_HASH);
        assert_eq!(user, db.get_user(DUPE_EMAIL).await.unwrap());
    }

    #[tokio::test]
    async fn test_set_node_owner_and_get_node() {
        let db = Database::new_in_memory().await.unwrap();

        let id = db.insert_node().await.unwrap();
        assert!(db.get_node(id, None).await.unwrap().is_some());
        assert!(db.get_node(id, Some(EMAIL)).await.unwrap().is_none());
        assert!(db.get_node(id, Some(DUPE_EMAIL)).await.unwrap().is_none());
        assert!(db.get_node(id, Some(WRONG_EMAIL)).await.unwrap().is_none());

        db.set_node_owner(id, Some(EMAIL), Some(EMAIL))
            .await
            .expect_err("Node owner was set when previous owner was incorrect");
        db.set_node_owner(id, None, Some(EMAIL)).await.unwrap();
        assert!(db.get_node(id, None).await.unwrap().is_some());
        assert!(db.get_node(id, Some(EMAIL)).await.unwrap().is_some());
        assert!(db.get_node(id, Some(DUPE_EMAIL)).await.unwrap().is_some());
        assert!(db.get_node(id, Some(WRONG_EMAIL)).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_set_node_owner_and_get_nodes() {
        let db = Database::new_in_memory().await.unwrap();

        let id = db.insert_node().await.unwrap();

        async fn assert_counts(db: &Database, count_unowned: usize, count_owned: usize) {
            assert_eq!(db.get_nodes(None).await.unwrap().len(), count_unowned);
            assert_eq!(db.get_nodes(Some(EMAIL)).await.unwrap().len(), count_owned);
            assert_eq!(
                db.get_nodes(Some(DUPE_EMAIL)).await.unwrap().len(),
                count_owned
            );
            assert_eq!(db.get_nodes(Some(WRONG_EMAIL)).await.unwrap().len(), 0);
        }

        // Check that there is 1 node with no owner
        assert_counts(&db, 1, 0).await;
        // Assign the owner
        db.set_node_owner(id, None, Some(EMAIL)).await.unwrap();
        // Check that there is 1 node with an owner
        assert_counts(&db, 0, 1).await;
        // Assign no owner
        db.set_node_owner(id, None, None)
            .await
            .expect_err("Node owner was set when previous owner was incorrect");
        db.set_node_owner(id, Some(EMAIL), None).await.unwrap();
        // Check that there is 1 node with no owner
        assert_counts(&db, 1, 0).await;
    }

    #[tokio::test]
    async fn test_set_node_config() {
        let db = Database::new_in_memory().await.unwrap();

        let id = db.insert_node().await.unwrap();

        // Test the initial values
        let node = db.get_node(id, None).await.unwrap().unwrap();
        assert_eq!(node.name, "");
        assert_eq!(node.latitude, 0.0);
        assert_eq!(node.latitude, 0.0);
        assert_eq!(node.empty_distance_reading, 1.0);
        assert_eq!(node.full_distance_reading, 0.0);

        let name = "Jeff";
        let (lat, long) = (1.0, 2.0);
        let (empty_distance_reading, full_distance_reading) = (1.5, 0.5);
        db.set_node_config(
            id,
            None,
            name,
            lat,
            long,
            empty_distance_reading,
            full_distance_reading,
        )
        .await
        .unwrap();
        let node = db.get_node(id, None).await.unwrap().unwrap();
        assert_eq!(node.name, name);
        assert_eq!(node.latitude, lat);
        assert_eq!(node.longitude, long);
        assert_eq!(node.empty_distance_reading, empty_distance_reading);
        assert_eq!(node.full_distance_reading, full_distance_reading);

        // Test filtering by owner email works
        db.set_node_owner(id, None, Some(EMAIL)).await.unwrap();
        db.set_node_config(
            id,
            None,
            name,
            lat,
            long,
            empty_distance_reading,
            full_distance_reading,
        )
        .await
        .unwrap();
        db.set_node_config(
            id,
            Some(EMAIL),
            name,
            lat,
            long,
            empty_distance_reading,
            full_distance_reading,
        )
        .await
        .unwrap();
        db.set_node_config(
            id,
            Some(DUPE_EMAIL),
            name,
            lat,
            long,
            empty_distance_reading,
            full_distance_reading,
        )
        .await
        .unwrap();
        db.set_node_config(
            id,
            Some(WRONG_EMAIL),
            name,
            lat,
            long,
            empty_distance_reading,
            full_distance_reading,
        )
        .await
        .expect_err("Setting node config by the wrong user was OK");
        db.set_node_config(id, Some(EMAIL), name, lat, long, 0.0, 0.0)
            .await
            .expect_err("Setting distance readings to a bad value was OK");
    }

    #[tokio::test]
    async fn test_set_node_fullness() {
        let db = Database::new_in_memory().await.unwrap();

        // (fullness, temperature, humidity)
        let data_points = [(0.0, 1.0, 3.0), (0.5, 2.0, 2.0), (1.0, 3.0, 1.0)];

        let mut ids = Vec::new();
        for _ in 0..data_points.len() {
            ids.push(db.insert_node().await.unwrap());
        }

        // Test the initial values
        let new_node = db.get_node(ids[0], None).await.unwrap().unwrap();
        assert_eq!(new_node.fullness, 0.0);
        assert_eq!(new_node.data_last_updated.naive_utc().timestamp(), 0);

        // Check that setting it actually sets it and updates the time
        for i in 0..data_points.len() {
            let data = data_points[i];
            db.set_node_data(ids[i], data.0, data.1, data.2)
                .await
                .unwrap();
            let node1 = db.get_node(ids[i], None).await.unwrap().unwrap();
            assert_eq!(node1.fullness, data.0);
            assert_eq!(node1.temperature, data.1);
            assert_eq!(node1.humidity, data.2);
            assert_ne!(node1.data_last_updated.naive_utc().timestamp(), 0);
        }

        // Reject bad values
        let invalid_fullnesses = [
            -1.0,
            -0.0000001,
            1.0000001,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        for fullness in invalid_fullnesses {
            db.set_node_data(ids[0], fullness, 0.0, 0.0)
                .await
                .expect_err(format!("Fullness {fullness} was OK").as_str());
        }
    }

    #[tokio::test]
    async fn test_set_get_user_last_email() {
        let db = Database::new_in_memory().await.unwrap();
        let id = db.insert_node().await.unwrap();
        db.insert_user(EMAIL, PASSWORD_HASH).await.unwrap();
        db.set_node_owner(id, None, Some(EMAIL)).await.unwrap();

        assert!(db.get_user_last_email_time(EMAIL).await.unwrap().is_none());
        let now = Utc::now();
        db.set_user_last_email_time(EMAIL, now).await.unwrap();
        assert_eq!(
            db.get_user_last_email_time(EMAIL).await.unwrap().unwrap(),
            now
        );
    }

    const EMAIL: &str = "TeSt@ExAmPlE.cOm";
    const DUPE_EMAIL: &str = "Test@Example.Com";
    const WRONG_EMAIL: &str = "Test@Example.co.uk";
    const PASSWORD_HASH: &str = "my.very.secure.definitely.hashed.password";
}
