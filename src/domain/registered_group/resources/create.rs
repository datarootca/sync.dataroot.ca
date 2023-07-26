use std::sync::Arc;

use crate::domain::{
    registered_group::{model::{RegisteredGroupModel,RegisteredGroupCreateModel}, repository::RegisteredGroupRepository},
    error::DomainError,
};

pub async fn execute(
    registered_group_repository: Arc<dyn RegisteredGroupRepository>,
    registered_group_create_model: RegisteredGroupCreateModel,
) -> Result<RegisteredGroupModel, DomainError> {
    let registered_group = registered_group_repository.insert(&registered_group_create_model).await?;
    Ok(registered_group)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

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
    async fn it_should_return_registered_group_created() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();

        registered_group_repository
            .expect_insert()
            .return_once(|_| Ok(RegisteredGroupModel::mock_default()));

        let result = execute(
            Arc::new(registered_group_repository),
            RegisteredGroupCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
