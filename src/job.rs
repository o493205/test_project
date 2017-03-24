use std::sync::Mutex;
use job_request::JobRequest;

#[derive(Debug)]
pub struct Job {
    pub job_id: String,
    pub job_request: Mutex<Option<JobRequest>>,
}

impl Job {
    pub fn validate(&self) -> bool {
        if self.job_id.trim() != "" {
            // * to deref MutexGuard<T> in order to use &Option
            match *self.job_request.lock().unwrap() {
                // ref used to get a reference to the JobRequest
                Some(ref jr) => {
                    if Job::validate_source(jr.clone()) && Job::validate_destination(jr.clone()) {
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
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
