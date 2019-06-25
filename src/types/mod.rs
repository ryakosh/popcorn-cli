pub mod candidates;

use crate::db::models;
use std::fmt;

pub enum CastJob {
    Writer,
    Director,
    Artist
}

impl fmt::Display for CastJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CastJob::Writer => write!(f, "writer"),
            CastJob::Director => write!(f, "director"),
            CastJob::Artist => write!(f, "artist"),
        }
    }
}

pub enum Casts {
    Writers(Vec<models::Writer>),
    Directors(Vec<models::Director>),
    Artists(Vec<models::Artist>),
}