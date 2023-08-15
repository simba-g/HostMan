use async_trait;

#[async_trait::async_trait]
pub trait ContainerManager {
    type Image;
    type ImageStatus;
    type ContainerId;
    type Container;
    type ContainerDefinition;
    type ContainerStats;
    type ContainerInspection;
    type ContainerLogs;
    type TimePeriod;
    type Error: std::error::Error;

    async fn list_images(&self) -> Result<Vec<String>, Self::Error>;
    async fn list_images_iter<T>(&self) -> Result<T, Self::Error>
        where T: Iterator<Item = Self::Image>;
    async fn pull_image(&self, image_uri: &str) -> Result<Self::ImageStatus, Self::Error>;
    async fn prune_images(&self) -> Result<Vec<String>, Self::Error>;

    async fn create(
        &self,
        container: Self::ContainerDefinition,
    ) -> Result<Self::ContainerId, Self::Error>;
    async fn start(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn stop(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn restart(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn remove(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn kill(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn inspect(&self, container_id: Self::ContainerId) -> Result<Self::ContainerInspection, Self::Error>;
    async fn logs(&self, container_id: Self::ContainerId, lines: Option<u32>, period: Option<Self::TimePeriod>) -> Result<Self::ContainerLogs, Self::Error>;
    async fn stats(&self, container_id: Self::ContainerId) -> Result<Self::ContainerStats, Self::Error>;
    async fn rename(&self, container_id: Self::ContainerId) -> Result<(), Self::Error>;
    async fn prune_containers(&self) -> Result<Vec<String>, Self::Error>;
    async fn list_containers(&self) -> Result<Vec<Self::ContainerStats>, Self::Error>;
    async fn list_containers_iter<T>(&self) -> Result<T, Self::Error>
        where T: Iterator<Item = Self::ContainerStats>;
}
