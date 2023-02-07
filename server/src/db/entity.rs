macro_rules! boilerplate {
    () => {
        use sea_orm::entity::prelude::*;
        impl ActiveModelBehavior for ActiveModel {}
    };
}

pub mod node {
    boilerplate!();

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "node")]
    pub struct Model {
        /// ID must be u32 due to a limitation of SeaORM's SQLite implementation
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: u32,
        pub name: String,
        pub owner: Option<String>,
        pub latitude: f64,
        pub longitude: f64,
        /// The distance reading above which the bin is considered to be empty
        pub empty_distance_reading: f32,
        /// The distance reading below which the bin is considered to be full
        pub full_distance_reading: f32,
        pub fullness: f32,
        pub fullness_last_updated: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::user::Entity",
            from = "Column::Owner",
            to = "super::user::Column::Email"
        )]
        User,
    }

    impl Related<super::user::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }
}

pub mod user {
    use chrono::DateTime;

    boilerplate!();

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub email: String,
        pub password_hash: String,
        pub last_email_time: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::node::Entity")]
        Node,
    }

    impl Related<super::node::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Node.def()
        }
    }
}
