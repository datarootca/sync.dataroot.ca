use std::sync::Arc;

use crate::domain::{
    registered_group::{model::RegisteredGroupModel, repository::RegisteredGroupRepository},
    error::DomainError,
};

pub async fn execute(
    registered_group_repository: Arc<dyn RegisteredGroupRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<RegisteredGroupModel>, u32)>, DomainError> {
    let registered_group = registered_group_repository.find(&name, &page, &page_size).await?;

    if registered_group.is_some() {
        return Ok(registered_group);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::domain::registered_group::model::{RegisteredGroupCreateModel};

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
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![RegisteredGroupModel::mock_default()], 1))));

        let (registered_group, count) = execute(Arc::new(registered_group_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!registered_group.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut registered_group_repository = MockFakeRegisteredGroupRepository::new();
        registered_group_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(registered_group_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
