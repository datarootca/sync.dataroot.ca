use std::sync::Arc;

use crate::domain::{
    diff_article::{model::{
        DiffArticleModel
    }, 
        repository::DiffArticleRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn DiffArticleRepository>,
    keys: &Vec<String>
) -> Result<Vec<DiffArticleModel>, DomainError> {
    let articles = article_repository.find_by_extids(keys.to_vec()).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::{api::lib::{BatchOperations, DiffOperations}};
    use crate::{domain::diff_article::model::{DiffArticleModel}, api::utils::random_string};

    mock! {
        pub FakeDiffArticleRepository { }

        #[async_trait]
        impl DiffArticleRepository for FakeDiffArticleRepository {
        }

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
            async fn find_by_extids(&self, _extids: Vec<String>) -> Result<Vec<DiffArticleModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut article_repository = MockFakeDiffArticleRepository::new();
        article_repository
            .expect_find_by_extids()
            .return_once(|_| Ok(vec![DiffArticleModel::mock_default()]));

        let articles = execute(Arc::new(article_repository), &vec![random_string(5)])
            .await
            .unwrap();

        assert!(!articles.is_empty());
    }
}
