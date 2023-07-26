use async_trait::async_trait;


use crate::{domain::error::DomainError, api::lib::BatchOperations};

use super::model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel};

#[async_trait]
pub trait ArticleRepository: BatchOperations<ArticleCreateModel,ArticleUpdateModel,ArticleModel> + Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
    async fn find_by_articleid(&self, id: &i32) -> Result<Option<ArticleModel>, DomainError>;
    async fn find_by_extid(&self, extid: &str) -> Result<Option<ArticleModel>, DomainError>;
    async fn insert(
        &self,
        article_create_model: &ArticleCreateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn update_by_extid(
        &self,
        article_update_model: &ArticleUpdateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn delete_by_articleid(&self, id: &i32) -> Result<(), DomainError>;
}