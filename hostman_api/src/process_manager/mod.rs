/// Process Manager
///
/// API describing the process manager trait for
/// host manager.
use async_trait;

#[async_trait::async_trait]
pub trait ProcessManager {
    type ProcessId;
    type ProcessStats;
    type ProcessStatsSortBy;
    type ProcessResource;
    type ProcessResourceLimits;
    type Error: std::error::Error;

    async fn spawn(&self, command: &str) -> Result<Self::ProcessId, Self::Error>;
    async fn signal(&self, pid: Self::ProcessId, signal: usize) -> Result<(), Self::Error>;
    async fn process_stats(&self, pid: Self::ProcessId) -> Result<Self::ProcessStats, Self::Error>;
    async fn all_stats(
        &self,
        sortby: Self::ProcessStatsSortBy,
    ) -> Result<Vec<Self::ProcessStats>, Self::Error>;
    async fn all_stats_iter<T>(&self, sortby: Self::ProcessStatsSortBy) -> Result<T, Self::Error>
    where
        T: Iterator<Item = Self::ProcessStats>;
    async fn get_limit(
        &self,
        resource: Self::ProcessResource,
    ) -> Result<Self::ProcessResourceLimits, Self::Error>;
    async fn set_limit(
        &self,
        resource: Self::ProcessResource,
        limits: Self::ProcessResourceLimits,
    ) -> Result<usize, Self::Error>;
}
