macro_rules! boilerplate {
    () => {
        use sea_orm::entity::prelude::*;
        impl ActiveModelBehavior for ActiveModel { }
    }
}

pub mod node {
    boilerplate!();

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "node")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: u64,
        pub owner: Option<String>,
        pub latitude: f64,
        pub longitude: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(belongs_to = "super::user::Entity", from = "Column::Owner", to = "super::user::Column::Email")]
        User,
    }

    impl Related<super::user::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }
}

pub mod user {
    boilerplate!();

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub email: String,
        pub password_hash: String,
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

