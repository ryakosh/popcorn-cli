pub mod schema;
pub mod models;
mod utils;

use crate::diesel;
use crate::diesel::prelude::*;
use crate::types;
use crate::types::candidates;
use std::env;

fn connect(conn_str: &str) -> PgConnection {
    PgConnection::establish(conn_str).expect(&format!("Error connecting to: {}", conn_str))
}

pub fn writers_query(term: &str) -> Vec<models::Writer> {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));

    diesel::sql_query(format!(
        include_str!("raw/cast_query.sql"),
        job=types::CastJob::Writer,
        term=term,
    )).load::<models::Writer>(&conn).expect("Error executing query")
}

pub fn directors_query(term: &str) -> Vec<models::Director> {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));

    diesel::sql_query(format!(
        include_str!("raw/cast_query.sql"),
        job=types::CastJob::Writer,
        term=term,
    )).load::<models::Director>(&conn).expect("Error executing query")
}

pub fn artists_query(term: &str) -> Vec<models::Artist> {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));

    diesel::sql_query(format!(
        include_str!("raw/cast_query.sql"),
        job=types::CastJob::Writer,
        term=term,
    )).load::<models::Artist>(&conn).expect("Error executing query")
}

pub fn cast_add(candidate: &candidates::CastCandidate) {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));
    
    diesel::sql_query(format!(
        include_str!("raw/cast_add.sql"),
        job=candidate.job(),
        first_name=candidate.first_name(),
        last_name=candidate.last_name(),
        gender=candidate.gender(),
    )).execute(&conn).expect("Error executing query");
}

pub fn movies_add(candidate: &candidates::MovieCandidate) {
    let conn = connect(&env::var("DATABASE_URL")
        .expect("Can't find DATABASE_URL environment variable"));
    
    let movie_id: models::MovieId = diesel::sql_query(format!(
        include_str!("raw/movies_add.sql"),
        title=candidate.title(),
        description=candidate.description(),
        poster=candidate.poster(),
        genres=candidate.genres(),
        languages=candidate.languages(),
        release_country=candidate.release_country(),
        release_date=candidate.release_date(),
        duration=candidate.duration(),
    )).get_result(&conn).expect("Error executing query");

    diesel::sql_query(format!(
        include_str!("raw/movies_add-writers.sql"),
        writers=utils::casts_to_values(movie_id.movie_id, candidate.writers()),
    )).execute(&conn).expect("Error executing query");

    diesel::sql_query(format!(
        include_str!("raw/movies_add-directors.sql"),
        directors=utils::casts_to_values(movie_id.movie_id, candidate.directors()),
    )).execute(&conn).expect("Error executing query");

    diesel::sql_query(format!(
        include_str!("raw/movies_add-artists.sql"),
        artists=utils::casts_to_values(movie_id.movie_id, candidate.artists()),
    )).execute(&conn).expect("Error executing query");
}