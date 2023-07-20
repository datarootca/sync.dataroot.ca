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
    id: i32,
    group_update_model: GroupUpdateModel,
) -> Result<GroupModel, DomainError> {
    let has_group = group_repository.find_by_groupid(&id).await?;
    if has_group.is_none() {
        return Err(DomainError::NotFound(String::from("Group id not found")));
    }

    let group = group_repository
        .update_by_groupid(&id, &group_update_model)
        .await?;

    Ok(group)
}

#[cfg(test)]
mod tests {
    use crate::{domain::group::model::GroupCreateModel, api::utils::random_number};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

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
    async fn it_should_return_group_updated() {
        let mut group_repository = MockFakeGroupRepository::new();

        let mock_group_model = GroupModel::mock_default();
        let mut mock_request_group_update = GroupUpdateModel::mock_default();
        mock_request_group_update.name = mock_group_model.name.clone();

        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(Some(mock_group_model)));

        group_repository
            .expect_update_by_groupid()
            .return_once(|_, _| Ok(GroupModel::mock_default()));

        let response = execute(
            Arc::new(group_repository),
            random_number().to_owned(),
            mock_request_group_update,
        )
        .await
        .unwrap();

        assert!(response.groupid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_group() {
        let mut group_repository = MockFakeGroupRepository::new();
        group_repository
            .expect_find_by_groupid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(group_repository),
            random_number(),
            GroupUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
