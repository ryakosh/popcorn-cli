use crate::db::schema::writers;

#[table_name = "writers"]
#[derive(QueryableByName, Debug)]
pub struct Writer {
    pub writer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}