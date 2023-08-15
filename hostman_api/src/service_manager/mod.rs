use async_trait;

#[async_trait::async_trait]
pub trait ServiceManager {
    type ServiceId;
    type ServiceStats;
    type ServiceModel;
    type Error: std::error::Error;

    async fn all_service_stats(&self) -> Result<Vec<Self::ServiceStats>, Self::Error>;
    async fn all_service_stats_iter<T>(&self) -> Result<T, Self::Error>
        where T: Iterator<Item = Result<Self::ServiceStats, Self::Error>>;
    async fn service_stats(&self, service_id: Self::ServiceId) -> Result<Self::ServiceStats, Self::Error>;
    async fn start_service(&self, service_id: Self::ServiceId) -> Result<(), Self::Error>;
    async fn stop_service(&self, service_id: Self::ServiceId) -> Result<(), Self::Error>;
    async fn restart_service(&self, service_id: Self::ServiceId) -> Result<(), Self::Error>;
    async fn create_service(&self, service: Self::ServiceModel) -> Result<(), Self::Error>;
}