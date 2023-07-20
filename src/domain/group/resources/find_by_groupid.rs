use std::sync::Arc;

use crate::domain::{
    group::{model::GroupModel, repository::GroupRepository},
    error::DomainError,
};

pub async fn execute(
    group_repository: Arc<dyn GroupRepository>,
    id: i32,
) -> Result<Option<GroupModel>, DomainError> {
    if let Some(group) = group_repository.find_by_groupid(&id).await? {
        return Ok(Some(group));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::group::model::{GroupCreateModel, GroupUpdateModel}, api::utils::random_number};

    use super::*;

    mock! {
        pub FakeGroupRepository { }

        #[async_trait]
        impl GroupRepository for FakeGroupRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError>;
            async fn find_by_groupid(&self, id: &i32) -> Result<Option<GroupModel>, DomainError>;
            async fn insert(&self,group_create_model: &GroupCreateModel) -> Result<GroupModel, DomainError>;
            async fn update_by_groupid(&self,id: &i32,group_update_model: &GroupUpdateModel) -> Result<GroupModel, DomainError>;
            async fn delete_by_groupid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_group_finded() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(Some(GroupModel::mock_default())));

        let result = execute(Arc::new(group_repository), random_number()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_group() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(group_repository), random_number()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
