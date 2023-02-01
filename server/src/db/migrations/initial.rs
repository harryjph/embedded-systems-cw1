use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "initial"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Node::Table)
                .col(
                    ColumnDef::new(Node::Id)
                        .big_unsigned()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Node::Latitude).double().not_null())
                .col(ColumnDef::new(Node::Longitude).double().not_null())
                .to_owned()
        ).await?;
        manager.create_table(
            Table::create()
                .table(User::Table)
                .col(
                    ColumnDef::new(User::Id)
                        .big_unsigned()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(User::Username).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null())
                .col(ColumnDef::new(User::PasswordHash).string().not_null())
                .col(ColumnDef::new(User::PasswordSalt).string().not_null())
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Node {
    Table,
    Id,
    Latitude,
    Longitude,
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    PasswordSalt,
}
