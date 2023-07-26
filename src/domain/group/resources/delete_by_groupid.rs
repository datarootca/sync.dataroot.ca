use std::sync::Arc;

use crate::domain::{group::repository::GroupRepository, error::DomainError};

pub async fn execute(
    group_repository: Arc<dyn GroupRepository>,
    group_id: i32,
) -> Result<(), DomainError> {
    let has_group = group_repository.find_by_groupid(&group_id).await?;
    if has_group.is_none() {
        return Err(DomainError::NotFound(String::from("Group id not found")));
    }

    group_repository.delete_by_groupid(&group_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::group::model::{
        GroupCreateModel, GroupModel, GroupUpdateModel,
    }, api::utils::random_number};

    use crate::api::lib::BatchOperations;
    use super::*;
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
    async fn it_should_return_void_group_deleted() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(Some(GroupModel::mock_default())));

        group_repository
            .expect_delete_by_groupid()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(group_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_group_not_found() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(group_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
