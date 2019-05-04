use crate::db::schema::*;

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

#[table_name = "artists"]
#[derive(QueryableByName)]
pub struct Artist {
    pub artist_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}

#[table_name = "movies_writers"]
#[derive(Insertable)]
pub struct NewMovieWriter {
    pub movie_id: i32,
    pub writer_id: i32,
}

#[table_name = "movies_directors"]
#[derive(Insertable)]
pub struct NewMovieDirector {
    pub movie_id: i32,
    pub director_id: i32,
}

#[table_name = "movies_artists"]
#[derive(Insertable)]
pub struct NewMovieArtist {
    pub movie_id: i32,
    pub artist_id: i32,
}

#[table_name = "movies"]
#[derive(QueryableByName)]
pub struct MovieId {
    pub movie_id: i32,
}