use async_trait::async_trait;
use crate::domain::error::DomainError;

use super::model::{ ArticleCreateModel};

#[async_trait]
pub trait ArticleAdapter {
    async fn fetch(&self, author: String) -> Result<Vec<ArticleCreateModel>, DomainError>;
}
