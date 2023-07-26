use std::sync::Arc;

use crate::domain::{
    group::{model::GroupModel, repository::GroupRepository},
    error::DomainError,
};

pub async fn execute(
    group_repository: Arc<dyn GroupRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError> {
    let group = group_repository.find(&name, &page, &page_size).await?;

    if group.is_some() {
        return Ok(group);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::domain::group::model::{GroupCreateModel, GroupUpdateModel};
    use crate::api::lib::BatchOperations;
    mock! {
        pub FakeGroupRepository { }

        #[async_trait]
        impl GroupRepository for FakeGroupRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError>;
            async fn find_by_extid(&self, extid: &str) -> Result<Option<GroupModel>, DomainError>;
            async fn find_by_groupid(&self, id: &i32) -> Result<Option<GroupModel>, DomainError>;
            async fn insert(&self,group_create_model: &GroupCreateModel) -> Result<GroupModel, DomainError>;
            async fn update_by_extid(&self,group_update_model: &GroupUpdateModel,) -> Result<GroupModel, DomainError>;
            async fn delete_by_groupid(&self, id: &i32) -> Result<(), DomainError>;
        }
        #[async_trait]
        impl BatchOperations<GroupCreateModel, GroupUpdateModel, GroupModel> for FakeGroupRepository {
            async fn insert_many(&self, _items: Vec<GroupCreateModel>) -> Result<Vec<GroupModel>, DomainError> {
                // Your implementation here...
            }
        
            async fn update_many(&self, _items: Vec<GroupUpdateModel>) -> Result<Vec<GroupModel>, DomainError> {
                // Your implementation here...
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_group_finded() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![GroupModel::mock_default()], 1))));

        let (group, count) = execute(Arc::new(group_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!group.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut group_repository = MockFakeGroupRepository::new();
        group_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(group_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
