use std::sync::Arc;

use crate::domain::{
    diff_event::{model::DiffEventModel, repository::DiffEventRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn DiffEventRepository>,
    article_update_models: Vec<DiffEventModel>,
) -> Result<Vec<DiffEventModel>, DomainError> {
    let articles = article_repository.update_many(article_update_models).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use crate::domain::diff_event::model::DiffEventModel;
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::{api::lib::{BatchOperations, DiffOperations}};
    mock! {
        pub FakeDiffEventRepository { }

        #[async_trait]
        impl DiffEventRepository for FakeDiffEventRepository {}

        #[async_trait]
        impl BatchOperations<DiffEventModel, DiffEventModel, DiffEventModel> for FakeDiffEventRepository {
            async fn insert_many(&self, _items: Vec<DiffEventModel>) -> Result<Vec<DiffEventModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }

            async fn update_many(&self, _items: Vec<DiffEventModel>) -> Result<Vec<DiffEventModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

        #[async_trait]
        impl DiffOperations<DiffEventModel> for FakeDiffEventRepository {
            async fn find_by_extids(&self, _items: Vec<String>) -> Result<Vec<DiffEventModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }
    }

    #[tokio::test]
    async fn article_created() {
        let mut article_repository = MockFakeDiffEventRepository::new();

        article_repository
            .expect_update_many()
            .return_once(|_| Ok(vec![DiffEventModel::mock_default()]));

        let result = execute(
            Arc::new(article_repository),
            vec![DiffEventModel::mock_default()],

        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
