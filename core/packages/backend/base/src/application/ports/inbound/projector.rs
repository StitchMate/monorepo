use async_trait::async_trait;

#[async_trait]
pub trait ProjectorHandler<T, E> where E: std::error::Error {
    async fn handle(&self, event: T) -> Result<(), E>;
}

#[async_trait]
pub trait Projector {
    async fn run(&self) -> Result<(), anyhow::Error>;
}