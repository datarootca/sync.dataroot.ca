use async_trait::async_trait;
use super::model::{ GroupCreateModel};

#[async_trait]
pub trait GroupAdapter {
    async fn fetch(&self, names: Vec<String>) -> Result<Vec<GroupCreateModel>, Box<dyn std::error::Error>>;
}