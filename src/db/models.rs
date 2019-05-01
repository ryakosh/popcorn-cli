use crate::db::schema::{writers, directors};

#[table_name = "writers"]
#[derive(QueryableByName)]
pub struct Writer {
    pub writer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}

#[table_name = "directors"]
#[derive(QueryableByName)]
pub struct Director {
    pub director_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}