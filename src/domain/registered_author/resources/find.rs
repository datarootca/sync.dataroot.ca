use std::sync::Arc;

use crate::domain::{
    registered_author::{model::RegisteredAuthorModel, repository::RegisteredAuthorRepository},
    error::DomainError,
};

pub async fn execute(
    registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<RegisteredAuthorModel>, u32)>, DomainError> {
    let registered_author = registered_author_repository.find(&name, &page, &page_size).await?;

    if registered_author.is_some() {
        return Ok(registered_author);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::domain::registered_author::model::{RegisteredAuthorCreateModel};

    mock! {
        pub FakeRegisteredAuthorRepository { }

        #[async_trait]
        impl RegisteredAuthorRepository for FakeRegisteredAuthorRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<RegisteredAuthorModel>, u32)>, DomainError>;
            async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredAuthorModel>, DomainError>;
            async fn insert(&self,registered_author_create_model: &RegisteredAuthorCreateModel) -> Result<RegisteredAuthorModel, DomainError>;
            async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_registered_author_finded() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();

        registered_author_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![RegisteredAuthorModel::mock_default()], 1))));

        let (registered_author, count) = execute(Arc::new(registered_author_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!registered_author.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();
        registered_author_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(registered_author_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
