use async_trait;
use std::path::Path;

#[async_trait::async_trait]
pub trait SecretsManager {
    type SecretSource;
    type SecretSignature;
    type Error: std::error::Error;

    async fn secret_signature(
        &self,
        secret_path: Path,
    ) -> Result<Self::SecretSignature, Self::Error>;
    async fn upsert_secret(
        &self,
        secret_path: Path,
        secret: Self::SecretSource,
    ) -> Result<(), Self::Error>;
    async fn delete_secret(&self, secret_path: Path) -> Result<(), Self::Error>;
    unsafe fn read_signature(&self, secret_path: Path) -> Result<Self::SecretSource, Self::Error>;
}
