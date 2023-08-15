use async_trait;

#[async_trait::async_trait]
pub trait JobManager {
    type Job;
    type JobId;
    type JobStatus;
    type Error: std::error::Error;

    async fn run(&self, job: Self::Job);
    async fn list(&self) -> Result<Vec<Self::JobId>, Self::Error>;
    async fn list_iter<T>(&self) -> Result<T, Self::Error>
        where T: Iterator<Item = Self::JobId>;
    async fn check(&self, job_id: Self::JobId) -> Option<Self::JobStatus>;
    async fn cancel(&self, job_id: Self::JobId) -> Result<Self::JobStatus, Self::Error>;
}
