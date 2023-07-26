use async_trait::async_trait;


use crate::{domain::error::DomainError};

use super::model::{RegisteredGroupCreateModel, RegisteredGroupModel};

#[async_trait]
pub trait RegisteredGroupRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<RegisteredGroupModel>, u32)>, DomainError>;
    async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredGroupModel>, DomainError>;
    async fn insert(
        &self,
        registered_group_create_model: &RegisteredGroupCreateModel,
    ) -> Result<RegisteredGroupModel, DomainError>;
    async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError>;
}
