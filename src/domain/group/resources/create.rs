use std::sync::Arc;

use crate::domain::group::model::GroupModel;
use crate::domain::{
    group::{model::GroupCreateModel, repository::GroupRepository},
    error::DomainError,
};

pub async fn execute(
    group_repository: Arc<dyn GroupRepository>,
    group_create_model: GroupCreateModel,
) -> Result<GroupModel, DomainError> {
    let group = group_repository.insert(&group_create_model).await?;
    Ok(group)
}

#[cfg(test)]
mod tests {
    use crate::domain::group::model::GroupUpdateModel;

    use super::*;

    use async_trait::async_trait;
    use crate::api::lib::BatchOperations;
    use mockall::mock;
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
    async fn it_should_return_group_created() {
        let mut group_repository = MockFakeGroupRepository::new();

        group_repository
            .expect_insert()
            .return_once(|_| Ok(GroupModel::mock_default()));

        let result = execute(
            Arc::new(group_repository),
            GroupCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
