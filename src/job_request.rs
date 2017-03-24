use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobType {
    FFprobe,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobRequest {
    pub job_type: JobType,
    pub source_path_to_directory: String,
    pub source_filename: String,
    pub destination_path_to_directory: String,
    pub destination_filename: String,
}
