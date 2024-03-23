use async_trait::async_trait;
use std::{error::Error, sync::Arc};

use super::event::Event;

pub trait Command {
    fn to_string(&self) -> String;
}

#[async_trait]
pub trait HandleCommand<C> {
    type Event: Event;
    type Error: Error;
    type Context;

    async fn handle(
        &self,
        command: C,
        context: Arc<Self::Context>,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}

#[async_trait]
pub trait VerifyCommand {
    type Context;
    type Error: Error;

    async fn verify(&mut self, context: Arc<Self::Context>) -> Result<(), Vec<Self::Error>>;
}
