use async_trait::async_trait;

use crate::{domain::error::DomainError, api::lib::BatchOperations};

use super::model::{GroupCreateModel, GroupModel, GroupUpdateModel};

#[async_trait]
pub trait GroupRepository: BatchOperations<GroupCreateModel,GroupUpdateModel,GroupModel> + Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError>;
    async fn find_by_groupid(&self, id: &i32) -> Result<Option<GroupModel>, DomainError>;
    async fn find_by_extid(&self, extid: &str) -> Result<Option<GroupModel>, DomainError>;
    async fn insert(
        &self,
        group_create_model: &GroupCreateModel,
    ) -> Result<GroupModel, DomainError>;
    async fn update_by_extid(
        &self,
        group_update_model: &GroupUpdateModel,
    ) -> Result<GroupModel, DomainError>;
    async fn delete_by_groupid(&self, id: &i32) -> Result<(), DomainError>;
}
