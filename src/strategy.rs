use super::types::{Action, Config, Event};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use ethers::providers::Middleware;
use std::sync::Arc;
pub struct ExampleStrat<M> {
    client: Arc<M>,
}
impl<M: Middleware + 'static> ExampleStrat<M> {
    pub fn new(client: Arc<M>, config: Config) -> Self {
        Self { client }
    }
}
#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for ExampleStrat<M> {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {}
    }
}
impl<M: Middleware + 'static> ExampleStrat<M> {}
