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
        manager
            .create_table(
                Table::create()
                    .table(Node::Table)
                    .col(
                        ColumnDef::new(Node::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Node::Name).string().not_null())
                    .col(ColumnDef::new(Node::Owner).string().null())
                    .col(ColumnDef::new(Node::Latitude).double().not_null())
                    .col(ColumnDef::new(Node::Longitude).double().not_null())
                    .col(ColumnDef::new(Node::Fullness).float().not_null())
                    .col(
                        ColumnDef::new(Node::FullnessLastUpdated)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Node {
    Table,
    Id,
    Name,
    Owner,
    Latitude,
    Longitude,
    Fullness,
    FullnessLastUpdated,
}

#[derive(Iden)]
pub enum User {
    Table,
    Email,
    PasswordHash,
}
