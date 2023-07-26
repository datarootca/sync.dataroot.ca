use async_trait::async_trait;


use crate::{domain::error::DomainError};

use super::model::{RegisteredAuthorCreateModel, RegisteredAuthorModel};

#[async_trait]
pub trait RegisteredAuthorRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<RegisteredAuthorModel>, u32)>, DomainError>;
    async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredAuthorModel>, DomainError>;
    async fn insert(
        &self,
        registered_author_create_model: &RegisteredAuthorCreateModel,
    ) -> Result<RegisteredAuthorModel, DomainError>;
    async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError>;
}
