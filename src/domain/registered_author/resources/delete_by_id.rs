use std::sync::Arc;



use crate::domain::{registered_author::repository::RegisteredAuthorRepository, error::DomainError};

pub async fn execute(
    registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
    registered_author_id: i32,
) -> Result<(), DomainError> {
    let has_registered_author = registered_author_repository.find_by_id(&registered_author_id).await?;
    if has_registered_author.is_none() {
        return Err(DomainError::NotFound(String::from("RegisteredAuthor id not found")));
    }

    registered_author_repository.delete_by_id(&registered_author_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::{domain::registered_author::model::{
        RegisteredAuthorCreateModel, RegisteredAuthorModel,
    }, api::utils::random_number};

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
    async fn it_should_return_void_registered_author_deleted() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();

        registered_author_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(RegisteredAuthorModel::mock_default())));

        registered_author_repository
            .expect_delete_by_id()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(registered_author_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_registered_author_not_found() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();

        registered_author_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(registered_author_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
