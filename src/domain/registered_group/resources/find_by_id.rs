use std::sync::Arc;



use crate::domain::{
    registered_group::{model::RegisteredGroupModel, repository::RegisteredGroupRepository},
    error::DomainError,
};

pub async fn execute(
    registered_group_repository: Arc<dyn RegisteredGroupRepository>,
    id: i32,
) -> Result<Option<RegisteredGroupModel>, DomainError> {
    if let Some(registered_group) = registered_group_repository.find_by_id(&id).await? {
        return Ok(Some(registered_group));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::registered_group::model::{RegisteredGroupCreateModel}, api::utils::random_number};

    use super::*;

    mock! {
        pub FakeRegisteredGroupRepository { }

        #[async_trait]
        impl RegisteredGroupRepository for FakeRegisteredGroupRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<RegisteredGroupModel>, u32)>, DomainError>;
            async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredGroupModel>, DomainError>;
            async fn insert(&self,registered_group_create_model: &RegisteredGroupCreateModel) -> Result<RegisteredGroupModel, DomainError>;
            async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_registered_group_finded() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();

        registered_group_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(RegisteredGroupModel::mock_default())));

        let result = execute(Arc::new(registered_group_repository), random_number()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_registered_group() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();

        registered_group_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(registered_group_repository), random_number()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
