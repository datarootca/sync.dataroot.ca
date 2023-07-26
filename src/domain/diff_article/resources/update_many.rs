use std::sync::Arc;

use crate::domain::{
    diff_article::{model::DiffArticleModel, repository::DiffArticleRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn DiffArticleRepository>,
    article_update_models: Vec<DiffArticleModel>,
) -> Result<Vec<DiffArticleModel>, DomainError> {
    let articles = article_repository.update_many(article_update_models).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use crate::domain::diff_article::model::DiffArticleModel;
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::{api::lib::{BatchOperations, DiffOperations}};
    mock! {
        pub FakeDiffArticleRepository { }

        #[async_trait]
        impl DiffArticleRepository for FakeDiffArticleRepository {}

        #[async_trait]
        impl BatchOperations<DiffArticleModel, DiffArticleModel, DiffArticleModel> for FakeDiffArticleRepository {
            async fn insert_many(&self, _items: Vec<DiffArticleModel>) -> Result<Vec<DiffArticleModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }

            async fn update_many(&self, _items: Vec<DiffArticleModel>) -> Result<Vec<DiffArticleModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

        #[async_trait]
        impl DiffOperations<DiffArticleModel> for FakeDiffArticleRepository {
            async fn find_by_extids(&self, _items: Vec<String>) -> Result<Vec<DiffArticleModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }
    }

    #[tokio::test]
    async fn article_created() {
        let mut article_repository = MockFakeDiffArticleRepository::new();

        article_repository
            .expect_update_many()
            .return_once(|_| Ok(vec![DiffArticleModel::mock_default()]));

        let result = execute(
            Arc::new(article_repository),
            vec![DiffArticleModel::mock_default()],

        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
