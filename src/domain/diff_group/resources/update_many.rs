use std::sync::Arc;

use crate::domain::{
    diff_group::{model::DiffGroupModel, repository::DiffGroupRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn DiffGroupRepository>,
    article_update_models: Vec<DiffGroupModel>,
) -> Result<Vec<DiffGroupModel>, DomainError> {
    let articles = article_repository.update_many(article_update_models).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use crate::domain::diff_group::model::DiffGroupModel;
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::{api::lib::{BatchOperations, DiffOperations}};
    mock! {
        pub FakeDiffGroupRepository { }

        #[async_trait]
        impl DiffGroupRepository for FakeDiffGroupRepository {}

        #[async_trait]
        impl BatchOperations<DiffGroupModel, DiffGroupModel, DiffGroupModel> for FakeDiffGroupRepository {
            async fn insert_many(&self, _items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }

            async fn update_many(&self, _items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

        #[async_trait]
        impl DiffOperations<DiffGroupModel> for FakeDiffGroupRepository {
            async fn find_by_extids(&self, _items: Vec<String>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }
    }

    #[tokio::test]
    async fn article_created() {
        let mut article_repository = MockFakeDiffGroupRepository::new();

        article_repository
            .expect_update_many()
            .return_once(|_| Ok(vec![DiffGroupModel::mock_default()]));

        let result = execute(
            Arc::new(article_repository),
            vec![DiffGroupModel::mock_default()],

        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
