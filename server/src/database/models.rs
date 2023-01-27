use diesel::prelude::*;

#[derive(Queryable)]
pub struct node {
    pub longitude: f64,
    pub latitude: f64,
}
