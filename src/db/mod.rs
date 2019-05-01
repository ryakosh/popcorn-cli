pub mod schema;
pub mod models;

use crate::diesel;
use crate::diesel::prelude::*;
use std::env::var;
use models::Writer;

fn connect(conn_str: &str) -> PgConnection {
    PgConnection::establish(conn_str).expect(&format!("Error connecting to: {}", conn_str))
}

pub fn writers_query(term: &str) -> Vec<Writer> {

    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let query = format!(
        include_str!("raw/writers_query.sql"),
        term
    );

    diesel::sql_query(query)
        .load(&conn)
        .expect("Error executing query")
}