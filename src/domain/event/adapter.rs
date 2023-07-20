use async_trait::async_trait;
use super::model::{ EventCreateModel};

#[async_trait]
pub trait EventAdapter {
    async fn fetch(&self, group_names: Vec<String>) -> Result<Vec<EventCreateModel>, Box<dyn std::error::Error>>;
}