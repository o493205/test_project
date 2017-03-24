use job_request::JobRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub job_id: String,
    pub job_request: JobRequest,
}

impl Job {
    pub fn validate(&self) -> bool {
        if self.job_id.trim() != "" && Job::validate_source(self.job_request.clone()) &&
           Job::validate_destination(self.job_request.clone()) {
            return true;
        } else {
            return false;
        }
    }

    fn validate_source(job_request: JobRequest) -> bool {
        if job_request.source_path_to_directory.trim() != "" &&
           job_request.source_filename.trim() != "" {
            return true;
        } else {
            return false;
        }
    }

    fn validate_destination(job_request: JobRequest) -> bool {
        if job_request.destination_path_to_directory.trim() != "" &&
           job_request.destination_filename.trim() != "" {
            return true;
        } else {
            return false;
        }
    }
}
