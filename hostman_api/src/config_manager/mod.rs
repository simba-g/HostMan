use async_trait;

#[async_trait::async_trait]
pub trait ConfigManager {
    type ConfigKey;
    type ConfigValue;
    type Error: std::error::Error;

    async fn get_config(&self, key: Self::ConfigKey) -> Result<Self::ConfigValue, Self::Error>;
    async fn get_config_multi<T, U>(&self, key: T) -> U
    where
        T: IntoIterator<Item = Self::ConfigKey>,
        U: IntoIterator<Item = (Self::ConfigKey, Result<Self::ConfigValue, Self::Error>)>;
    async fn set_config(
        &self,
        key: Self::ConfigKey,
        value: Self::ConfigValue,
    ) -> Result<(), Self::Error>;
    async fn set_config_multi<T, U>(&self, configs: T) -> U
    where
        T: IntoIterator<Item = (Self::ConfigKey, Self::ConfigValue)>,
        U: IntoIterator<Item = (Self::ConfigKey, Result<(), Self::Error>)>;
}
