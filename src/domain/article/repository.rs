use async_trait::async_trait;


use crate::domain::error::DomainError;

use super::model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel};

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
    async fn find_by_articleid(&self, id: &i32) -> Result<Option<ArticleModel>, DomainError>;
    async fn insert(
        &self,
        article_create_model: &ArticleCreateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn update_by_articleid(
        &self,
        id: &i32,
        article_update_model: &ArticleUpdateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn delete_by_articleid(&self, id: &i32) -> Result<(), DomainError>;
}
