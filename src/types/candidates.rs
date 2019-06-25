use crate::types::CastJob;

pub struct CastCandidate<'c> {
    job: CastJob,
    first_name: &'c str,
    last_name: &'c str,
    gender: &'c str,
}

impl<'c> CastCandidate<'c> {
    pub fn new(job: CastJob, first_name: &'c str, 
               last_name: &'c str, gender: &'c str) -> CastCandidate<'c> {
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
        self.first_name
    }

    pub fn last_name(&self) -> &str {
        self.last_name
    }

    pub fn gender(&self) -> &str {
        self.gender
    }
}

pub struct MovieCandidate<'m> {
    title: &'m str,
    description: &'m str,
    poster: &'m str,
    genres: &'m str,
    languages: &'m str,
    release_country: &'m str,
    release_date: &'m str,
    duration: &'m str,
    writers: &'m str,
    directors: &'m str,
    artists: &'m str,
}

impl<'m> MovieCandidate<'m> {
    pub fn new(title: &'m str, description: &'m str, poster: &'m str,
               genres: &'m str, languages: &'m str, release_country: &'m str,
               release_date: &'m str, duration: &'m str,
               writers: &'m str, directors: &'m str, artists: &'m str) -> MovieCandidate<'m> {
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
        self.title
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn poster(&self) -> &str {
        self.poster
    }

    pub fn genres(&self) -> &str {
        self.genres
    }

    pub fn languages(&self) -> &str {
        self.languages
    }

    pub fn release_country(&self) -> &str {
        self.release_country
    }

    pub fn release_date(&self) -> &str {
        self.release_date
    }

    pub fn duration(&self) -> &str {
        self.duration
    }

    pub fn writers(&self) -> &str {
        self.writers
    }

    pub fn directors(&self) -> &str {
        self.directors
    }

    pub fn artists(&self) -> &str {
        self.artists
    }
}