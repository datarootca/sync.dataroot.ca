use std::sync::Arc;



use crate::domain::{
    registered_author::{model::RegisteredAuthorModel, repository::RegisteredAuthorRepository},
    error::DomainError,
};

pub async fn execute(
    registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
    id: i32,
) -> Result<Option<RegisteredAuthorModel>, DomainError> {
    if let Some(registered_author) = registered_author_repository.find_by_id(&id).await? {
        return Ok(Some(registered_author));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::registered_author::model::{RegisteredAuthorCreateModel}, api::utils::random_number};

    use super::*;

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
            .expect_find_by_id()
            .return_once(|_| Ok(Some(RegisteredAuthorModel::mock_default())));

        let result = execute(Arc::new(registered_author_repository), random_number()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_registered_author() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();

        registered_author_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(registered_author_repository), random_number()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
