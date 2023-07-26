use async_trait::async_trait;
use crate::domain::error::DomainError;

use super::model::{ GroupCreateModel};

#[async_trait]
pub trait GroupAdapter {
    async fn fetch(&self, names: Vec<String>) -> Result<Vec<GroupCreateModel>, DomainError>;
}