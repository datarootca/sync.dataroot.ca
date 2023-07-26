use std::sync::Arc;



use crate::domain::{registered_group::repository::RegisteredGroupRepository, error::DomainError};

pub async fn execute(
    registered_group_repository: Arc<dyn RegisteredGroupRepository>,
    registered_group_id: i32,
) -> Result<(), DomainError> {
    let has_registered_group = registered_group_repository.find_by_id(&registered_group_id).await?;
    if has_registered_group.is_none() {
        return Err(DomainError::NotFound(String::from("RegisteredGroup id not found")));
    }

    registered_group_repository.delete_by_id(&registered_group_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::{domain::registered_group::model::{
        RegisteredGroupCreateModel, RegisteredGroupModel,
    }, api::utils::random_number};

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
    async fn it_should_return_void_registered_group_deleted() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();

        registered_group_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(RegisteredGroupModel::mock_default())));

        registered_group_repository
            .expect_delete_by_id()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(registered_group_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_registered_group_not_found() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();

        registered_group_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(registered_group_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
