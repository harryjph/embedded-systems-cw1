use crate::database::schema::nodes;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Node {
    pub longitude: f64,
    pub latitude: f64,
}

/// We may insert into the database with fewer values. Node may
/// have an id which is defaulted in the future for example
#[derive(Insertable)]
#[diesel(table_name = nodes)]
pub struct NewNode {
    pub longitude: f64,
    pub latitude: f64,
}
