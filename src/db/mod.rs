pub mod schema;
pub mod models;

use crate::diesel;
use crate::diesel::prelude::*;
use crate::types;
use std::env;

fn connect(conn_str: &str) -> PgConnection {
    PgConnection::establish(conn_str).expect(&format!("Error connecting to: {}", conn_str))
}

pub fn cast_query(cast: types::CastJob , term: &str) -> types::Casts {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));
    let query = format!(
        include_str!("raw/cast_query.sql"),
        cast=cast,
        term=term,
    );
    let query_result = diesel::sql_query(query);

    match cast {
        types::CastJob::Writer => 
            types::Casts::Writers(query_result
                .load::<models::Writer>(&conn)
                .expect("Error executing query")),
        types::CastJob::Director => 
            types::Casts::Directors(query_result
                .load::<models::Director>(&conn)
                .expect("Error executing query")),
    }
}