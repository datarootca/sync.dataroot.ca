use std::sync::Arc;



use crate::domain::{
    group::{
        model::{GroupModel, GroupUpdateModel},
        repository::GroupRepository,
    },
    error::DomainError,
};

pub async fn execute(
    group_repository: Arc<dyn GroupRepository>,
    group_update_model: GroupUpdateModel,
) -> Result<GroupModel, DomainError> {
    let has_group = group_repository.find_by_extid(&group_update_model.extid).await?;
    if has_group.is_none() {
        return Err(DomainError::NotFound(String::from("Group id not found")));
    }

    let group = group_repository
        .update_by_extid( &group_update_model)
        .await?;

    Ok(group)
}

#[cfg(test)]
mod tests {
    use crate::{domain::group::model::GroupCreateModel};

    use super::*;
    use crate::api::lib::BatchOperations;
    use async_trait::async_trait;
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
    async fn it_should_return_group_updated() {
        let mut group_repository = MockFakeGroupRepository::new();

        let mock_group_model = GroupModel::mock_default();
        let mut mock_request_group_update = GroupUpdateModel::mock_default();
        mock_request_group_update.name = mock_group_model.name.clone();
        mock_request_group_update.extid = mock_group_model.extid.clone();

        group_repository
            .expect_find_by_extid()
            .return_once(|_| Ok(Some(mock_group_model)));

        group_repository
            .expect_update_by_extid()
            .return_once(|_| Ok(GroupModel::mock_default()));

        let response = execute(
            Arc::new(group_repository),
            mock_request_group_update,
        )
        .await
        .unwrap();

        assert!(!response.extid.is_empty());
    }
}
