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
        pub latitude: f64,
        pub longitude: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation { }
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
    pub enum Relation { }
}

