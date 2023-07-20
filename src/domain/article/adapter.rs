use async_trait::async_trait;
use super::model::{ ArticleCreateModel};

#[async_trait]
pub trait ArticleAdapter {
    async fn fetch(&self, author: String) -> Result<Vec<ArticleCreateModel>, Box<dyn std::error::Error>>;
}
