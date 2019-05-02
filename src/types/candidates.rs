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