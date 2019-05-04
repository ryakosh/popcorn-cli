use crate::types::CastJob;

pub struct CastCandidate {
    job: CastJob,
    first_name: String,
    last_name: String,
    gender: String,
}

impl CastCandidate {
    pub fn new(job: CastJob, first_name: String, 
               last_name: String, gender: String) -> CastCandidate {
        CastCandidate {
            job,
            first_name,
            last_name,
            gender,
        }
    }

    pub fn job(&self) -> &CastJob {
        &self.job
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn gender(&self) -> &str {
        &self.gender
    }
}

pub struct MovieCandidate {
    title: String,
    description: String,
    poster: String,
    genres: String,
    languages: String,
    release_country: String,
    release_date: String,
    duration: String,
    writers: String,
    directors: String,
    artists: String,
}

impl MovieCandidate {
    pub fn new(title: String, description: String, poster: String,
               genres: String, languages: String, release_country: String,
               release_date: String, duration: String,
               writers: String, directors: String, artists: String) -> MovieCandidate {
        MovieCandidate {
            title,
            description,
            poster,
            genres,
            languages,
            release_country,
            release_date,
            duration,
            writers,
            directors,
            artists,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn poster(&self) -> &str {
        &self.poster
    }

    pub fn genres(&self) -> &str {
        &self.genres
    }

    pub fn languages(&self) -> &str {
        &self.languages
    }

    pub fn release_country(&self) -> &str {
        &self.release_country
    }

    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    pub fn duration(&self) -> &str {
        &self.duration
    }

    pub fn writers(&self) -> &str {
        &self.writers
    }

    pub fn directors(&self) -> &str {
        &self.directors
    }

    pub fn artists(&self) -> &str {
        &self.artists
    }
}