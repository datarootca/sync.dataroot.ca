use async_trait::async_trait;
use crate::domain::{error::DomainError, group::model::GroupModel};

use super::model::{ EventCreateModel};

#[async_trait]
pub trait EventAdapter {
    async fn fetch(&self, group_models: Vec<GroupModel>) -> Result<Vec<EventCreateModel>, DomainError>;
}