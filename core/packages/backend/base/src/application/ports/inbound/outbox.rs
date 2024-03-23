use async_trait::async_trait;

#[async_trait]
pub trait Outbox {
    async fn run(&self) -> Result<(), anyhow::Error>;
}