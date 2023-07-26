use std::sync::Arc;

use crate::domain::{
    registered_author::{model::{RegisteredAuthorModel,RegisteredAuthorCreateModel}, repository::RegisteredAuthorRepository},
    error::DomainError,
};

pub async fn execute(
    registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
    registered_author_create_model: RegisteredAuthorCreateModel,
) -> Result<RegisteredAuthorModel, DomainError> {
    let registered_author = registered_author_repository.insert(&registered_author_create_model).await?;
    Ok(registered_author)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

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
    async fn it_should_return_registered_author_created() {
        let mut registered_author_repository = MockFakeRegisteredAuthorRepository::new();

        registered_author_repository
            .expect_insert()
            .return_once(|_| Ok(RegisteredAuthorModel::mock_default()));

        let result = execute(
            Arc::new(registered_author_repository),
            RegisteredAuthorCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
