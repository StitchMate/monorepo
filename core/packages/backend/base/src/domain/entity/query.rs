use async_trait::async_trait;
use std::{error::Error, sync::Arc};

pub trait Query {
    fn to_string(&self) -> String;
}

#[async_trait]
pub trait VerifyQuery {
    type Context;
    type Error: Error;

    async fn verify(&mut self, context: Arc<Self::Context>) -> Result<(), Self::Error>;
}
