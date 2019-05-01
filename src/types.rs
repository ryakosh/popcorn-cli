use crate::db::models;
use std::fmt;

pub enum CastJob {
    Writer,
    Director,
}

impl fmt::Display for CastJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CastJob::Writer => write!(f, "writer"),
            CastJob::Director => write!(f, "director"),
        }
    }
}

pub enum Casts {
    Writers(Vec<models::Writer>),
    Directors(Vec<models::Director>),
}